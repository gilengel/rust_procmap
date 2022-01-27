use geo::{Coordinate, Rect};
use rust_editor::{InformationLayer, camera::Camera, interactive_element::{InteractiveElementState, InteractiveElement}, renderer::PrimitiveRenderer, style::Style};

use crate::{
    state::System,
    actions::action::Action, map::map::Map,
};

fn default_coordinate() -> Coordinate<f64> {
    Coordinate { x: 0., y: 0. }
}

pub struct BoxSelectSystem {
    selection_min: Coordinate<f64>,
    selection_max: Coordinate<f64>,
    active: bool,
}

impl BoxSelectSystem {
    pub fn new() -> Self {
        BoxSelectSystem::default()
    }
}

impl Default for BoxSelectSystem {
    fn default() -> Self {
        BoxSelectSystem {
            selection_min: default_coordinate(),
            selection_max: default_coordinate(),
            active: false,
        }
    }
}

impl System for BoxSelectSystem {
    fn mouse_down(&mut self, mouse_pos: Coordinate<f64>, _: u32, _: &mut Map, _actions: &mut Vec<Box<dyn Action>>) {
        self.selection_min = mouse_pos;
        self.active = true;
    }

    fn mouse_move(&mut self, mouse_pos: Coordinate<f64>, _: &mut Map, _actions: &mut Vec<Box<dyn Action>>) {
        self.selection_max = mouse_pos;
    }

    fn mouse_up(&mut self, _mouse_pos: Coordinate<f64>, _: u32, map: &mut Map, _actions: &mut Vec<Box<dyn Action>>) {
        for intersection in map
            .intersections_within_rectangle_mut(&Rect::new(self.selection_min, self.selection_max))
        {
            intersection.set_state(InteractiveElementState::Selected);
        }

        self.selection_min = default_coordinate();
        self.selection_max = default_coordinate();
        self.active = false;
    }

    fn enter(&mut self, _: &mut Map) {}

    fn exit(&self, _: &mut Map) {}

    fn render(
        &self,
        _map: &Map,
        context: &web_sys::CanvasRenderingContext2d,
        _additional_information_layer: &Vec<InformationLayer>,
        _camera: &Camera,
    ) -> Result<(), wasm_bindgen::JsValue> {

        if self.active {
            Rect::new(self.selection_min, self.selection_max).render(
                &Style {
                    border_width: 2,
                    border_color: "rgba(255, 255, 255, 0.1)".to_string(),
                    background_color: "rgba(255, 255, 255, 0.05)".to_string(),
                },
                context,
            )?;
        }

        Ok(())
    }
}
