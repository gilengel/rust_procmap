use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::euclidean_length::EuclideanLength;
use geo::line_intersection::line_intersection;
use geo::line_intersection::LineIntersection;
use geo::prelude::EuclideanDistance;
use geo::LineString;
use geo::Polygon;
use geo_types::Coordinate;
use geo_types::Line;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use geo_types::Point;

use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
#[derive(Debug, Clone)]
struct Intersection {
    position: Coordinate<f64>,
}

impl Intersection {
    pub fn set_position(&mut self, position: Coordinate<f64>) {
        self.position = position;
    }

    pub fn get_position(&self) -> Coordinate<f64> {
        self.position
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection {
            position: Coordinate { x: 0., y: 0. },
        }
    }
}

#[derive(Clone)]
struct Street {
    line: Line<f64>,
    polygon: Polygon<f64>,

    width: f64,

    start: Option<Rc<RefCell<Intersection>>>,
    end: Option<Rc<RefCell<Intersection>>>,
}

impl Default for Street {
    fn default() -> Self {
        Street {
            line: Line::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0)),
            width: 20.0,
            polygon: Polygon::new(LineString::from(vec![Coordinate { x: 0., y: 0. }]), vec![]),
            start: None,
            end: None,
        }
    }
}

impl Street {
    pub fn start(&self) -> Coordinate<f64> {
        self.start.as_ref().unwrap().borrow().get_position()
    }

    pub fn set_start(&mut self, start: Rc<RefCell<Intersection>>) {
        self.start = Some(start);

        self.line.start = self.start();
    }

    pub fn end(&self) -> Coordinate<f64> {
        self.end.as_ref().unwrap().borrow().get_position()
    }

    pub fn set_end(&mut self, end: Rc<RefCell<Intersection>>) {
        self.end = Some(end);
        self.line.end = self.end();

        self.update_geometry();
    }

    pub fn update_geometry(&mut self) {
        let half_width = self.width / 2.0;
        let start: Point<f64> = self.start().into();
        let end: Point<f64> = self.end().into();

        self.line.start = start.into();
        self.line.end = end.into();

        let length = start.euclidean_distance(&end);
        let vec = self.end() - self.start();
        let norm = Point::new(vec.x / length, vec.y / length);
        let perp = Point::new(-norm.y(), norm.x());
        let offset = perp * half_width;

        self.polygon = Polygon::new(
            LineString::from(vec![
                start - offset,
                start + norm * length - offset,
                start + norm * length + offset,
                start + offset,
            ]),
            vec![],
        );
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        let mut it = self.polygon.exterior().points_iter();
        let start = it.next().unwrap();

        context.begin_path();
        context.move_to(start.x(), start.y());
        for point in it {
            context.line_to(point.x(), point.y());
        }

        context.close_path();
        //context.fill();
        context.stroke();
    }

    pub fn intersect_with_street(&self, another: &Street) -> Option<LineIntersection<f64>> {
        line_intersection(self.line, another.line)
    }
}

#[wasm_bindgen]
pub struct Editor {
    width: u32,
    height: u32,
    streets: Vec<Street>,

    temp_street: Street,
    temp_start: Rc<RefCell<Intersection>>,
    temp_end: Rc<RefCell<Intersection>>,

    context: CanvasRenderingContext2d,

    mouse_pressed: bool,
}

fn get_canvas_and_context() -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("map_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok((canvas, context))
}

#[wasm_bindgen]
impl Editor {
    pub fn new() -> Editor {
        let (_, context) = get_canvas_and_context().unwrap();
        Editor {
            width: 1920,
            height: 800,
            streets: vec![],
            temp_street: Street::default(),
            context,
            mouse_pressed: false,
            temp_start: Rc::new(RefCell::new(Intersection::default())),
            temp_end: Rc::new(RefCell::new(Intersection::default())),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self) {
        self.context
            .clear_rect(0.0, 0.0, self.width.into(), self.height.into());

        self.temp_street.render(&self.context);

        for street in &self.streets {
            street.render(&self.context);
        }
    }

    pub fn mouse_down(&mut self, x: u32, y: u32) {
        {
            let mut start = self.temp_start.as_ref().borrow_mut();
            start.set_position(Coordinate {
                x: x.into(),
                y: y.into(),
            });
        }

        {
            let mut end = self.temp_end.as_ref().borrow_mut();
            end.set_position(Coordinate {
                x: x.into(),
                y: y.into(),
            });
        }

        self.temp_street.set_start(Rc::clone(&(self.temp_start)));
        self.temp_street.set_end(Rc::clone(&(self.temp_end)));

        self.mouse_pressed = true
    }

    pub fn mouse_up(&mut self, x: u32, y: u32) {
        if self.mouse_pressed {
            //self.temp_street.set_end(Coordinate { x: x.into(), y: y.into() });

            let mut new_street = self.temp_street.clone();
            let new_start = (*self.temp_start.as_ref().borrow()).clone();
            new_street.start = Some(Rc::new(RefCell::new(new_start)));

            let new_end = (*self.temp_end.as_ref().borrow()).clone();
            new_street.end = Some(Rc::new(RefCell::new(new_end)));

            self.streets.push(new_street);

            self.temp_street = Street::default();
        }

        self.mouse_pressed = false;
    }

    pub fn mouse_move(&mut self, x: u32, y: u32) {
        if self.mouse_pressed {
            {
                let mut end = self.temp_end.as_ref().borrow_mut();
                end.set_position(Coordinate {
                    x: x.into(),
                    y: y.into(),
                });
                //
            }
            self.temp_street.update_geometry();
            {
                let mut end = self.temp_end.as_ref().borrow_mut();
                match self.intersect_with_streets() {
                    Some(intersection) => end.set_position(intersection),
                    None => {}
                }
            }

            self.temp_street.update_geometry();
        }
    }

    fn intersect_with_streets(&self) -> Option<Coordinate<f64>> {
        let mut intersections = vec![];

        for street in &self.streets {
            if let Some(line_intersection) = self.temp_street.intersect_with_street(street) {
                match line_intersection {
                    LineIntersection::SinglePoint {
                        intersection,
                        is_proper,
                    } => {
                        if is_proper {
                            intersections.push(intersection);
                        }
                    }
                    _ => {}
                }
            }
        }

        intersections.sort_by(|a, b| {
            let d1 = a.euclidean_distance(&self.temp_street.start());
            let d2 = b.euclidean_distance(&self.temp_street.start());

            if d1 < d2 {
                return Ordering::Less;
            }

            if d1 == d2 {
                return Ordering::Equal;
            }

            Ordering::Greater
        });

        if intersections.is_empty() {
            return None;
        }

        Some(intersections.first().unwrap().clone())
    }
}