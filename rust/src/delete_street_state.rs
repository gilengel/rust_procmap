use std::{cell::RefCell, rc::Rc};

use geo::Coordinate;

use crate::{
    map::Map,
    state::State,
    street::Street,
    Renderer,
};

pub struct DeleteStreetState {
    hovered_street: Option<Rc<RefCell<Street>>>,
}

impl DeleteStreetState {
    pub fn new() -> Self {
        DeleteStreetState {
            hovered_street: None,
        }
    }
}

impl Default for DeleteStreetState {
    fn default() -> DeleteStreetState {
        DeleteStreetState {
            hovered_street: None,
        }
    }
}

impl State for DeleteStreetState {
    fn mouse_down(&mut self, _mouse_pos: Coordinate<f64>, _: u32, _: &mut Map) {}

    fn mouse_move(&mut self, _mouse_pos: Coordinate<f64>, _map: &mut Map) {
        /*

        if let Some(old_hovered_street) = &self.hovered_street {
            old_hovered_street
                .borrow_mut()
                .set_state(InteractiveElementState::Normal);
        }

        if let Some(hovered_street) = map.get_street_at_position(&mouse_pos) {
            {
                hovered_street
                    .borrow_mut()
                    .set_state(InteractiveElementState::Hover);
            }
            self.hovered_street = Some(Rc::clone(&hovered_street));
        }
        */
    }

    fn mouse_up(&mut self, _mouse_pos: Coordinate<f64>, _: u32, _map: &mut Map) {
        /*
        if let Some(hovered_street) = map.get_street_at_position(&position) {
            map.remove_street(Rc::clone(&hovered_street));
            self.hovered_street = None
        }
        */
    }

    fn enter(&self, _map: &mut Map) {}

    fn exit(&self, _map: &mut Map) {}

    fn render(
        &self,
        map: &Map,
        context: &web_sys::CanvasRenderingContext2d,
    ) -> Result<(), wasm_bindgen::JsValue> {
        context.clear_rect(0.0, 0.0, map.width().into(), map.height().into());

        map.render(context)?;

        if let Some(hovered_street) = &self.hovered_street {
            let hovered_street = hovered_street.borrow();
            //hovered_street.set_fillstyle("#FF0000");
            hovered_street.render(context)?;
        }

        Ok(())
    }
}
