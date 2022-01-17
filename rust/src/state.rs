use geo::Coordinate;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{Map, Renderer, map::InformationLayer, Camera};



/// Editing functionality is encapsuled into different states. Each state is responsible to render the map and all additional information needed. 
/// A state receives all input events that happen on the canvas element which are cursor down, up and move, key down and up.
/// 
/// Sometimes it is needed to create temporarily data to fullfill certain functionality while the state is active. Use the enter function to
/// prepare your state and the exit function to clean temporarily created data. Always ensure that the map is clean at the end of the exit function
/// and not necessary data is removed from the map. 
pub trait State {
    /// Is used to implement behaviour of the state if the user clicked inside the specified
    /// html element by the statemachine.
    ///
    /// * `x` - x coordinate of the cursor where the click occured
    /// * `y` - x coordinate of the cursor where the click occured
    /// * `button` - The number of the pressed button (0=left, 1=middle, 2=right) [See here for more informations](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    fn mouse_down(&mut self, mouse_pos: Coordinate<f64>, button: u32, map: &mut Map);

    /// Is used to implement behaviour of the state if the user moved the cursor inside the
    /// specified html element by the statemaschine.
    ///
    /// * `x` - x coordinate of the cursor where the click occured
    /// * `y` - x coordinate of the cursor where the click occured
    fn mouse_move(&mut self, mouse_pos: Coordinate<f64>, map: &mut Map);

    /// Is used to implement behaviour of the state if the user released a pressed mouse button
    /// inside the specified html element by the statemachine.
    ///
    /// * `x` - x coordinate of the cursor where the click occured
    /// * `y` - x coordinate of the cursor where the click occured
    /// * `button` - The number of the pressed button (0=left, 1=middle, 2=right) [See here for more informations](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    fn mouse_up(&mut self, mouse_pos: Coordinate<f64>, button: u32, map: &mut Map);

    fn render(&self, map: &Map, context: &CanvasRenderingContext2d, additional_information_layer: &Vec<InformationLayer>, camera: &Camera) -> Result<(), JsValue> {
        map.render(context, additional_information_layer, camera)?;

        Ok(())
    }

    /// Called every time the state is activated by the state maschine. Use it to 
    /// initialize values for the state.
    fn enter(&self, map: &mut Map);

    /// Called every time the state is deactivated by the state maschine. Use it to
    /// clean up values in the state.
    fn exit(&self, map: &mut Map);
}
