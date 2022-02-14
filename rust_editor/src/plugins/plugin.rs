use std::any::Any;

use core::hash::Hash;
use geo::Coordinate;
use web_sys::CanvasRenderingContext2d;
use yew::{Context, Html};

use crate::ui::app::App;

use super::camera::Renderer;

pub trait PluginWithOptions<Data, Modes>: Plugin<Data>
where
    Data: Renderer + Default + 'static,
    Modes: Clone + PartialEq + Eq + Hash + 'static,
{
    fn view_options(&self, context: &Context<App<Data, Modes>>) -> Html;

    fn update_property(&mut self, property: &str, value: Box<dyn Any>);

    fn identifier() -> &'static str
    where
        Self: Sized;
}

pub trait Plugin<Data>
where
    Data: Renderer + 'static,
{
    /// Is used to implement behaviour of the state if the user clicked inside the specified
    /// html element by the statemachine.
    ///
    /// * `x` - x coordinate of the cursor where the click occured
    /// * `y` - x coordinate of the cursor where the click occured
    /// * `button` - The number of the pressed button (0=left, 1=middle, 2=right) [See here for more informations](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    fn mouse_down(&mut self, _mouse_pos: Coordinate<f64>, _button: u32, _data: &mut Data) {}

    /// Is used to implement behaviour of the state if the user moved the cursor inside the
    /// specified html element by the statemaschine.
    ///
    /// * `x` - x coordinate of the cursor where the click occured
    /// * `y` - x coordinate of the cursor where the click occured
    fn mouse_move(
        &mut self,
        _mouse_pos: Coordinate<f64>,
        _mouse_movement: Coordinate<f64>,
        _data: &mut Data,
    ) {
    }

    fn render(&self, _context: &CanvasRenderingContext2d) {}

    /// Is used to implement behaviour of the state if the user released a pressed mouse button
    /// inside the specified html element by the statemachine.
    ///
    /// * `x` - x coordinate of the cursor where the click occured
    /// * `y` - x coordinate of the cursor where the click occured
    /// * `button` - The number of the pressed button (0=left, 1=middle, 2=right) [See here for more informations](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    fn mouse_up(&mut self, _mouse_pos: Coordinate<f64>, _button: u32, _data: &mut Data) {}

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
