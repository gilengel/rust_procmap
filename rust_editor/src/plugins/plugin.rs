use std::any::Any;

use core::hash::Hash;
use geo::Coordinate;
use web_sys::CanvasRenderingContext2d;
use yew::{html, Context, Html};

use crate::ui::app::App;
use crate::plugins::camera::Renderer;

pub trait AnyPlugin<Data>: Plugin<Data>
where
    Data: Renderer + Default + 'static,
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(PartialEq)]
pub enum SpecialKey {
    Ctrl,
    Shift,
    Alt
}

/// Option trait for a plugin.
///
/// It provides functions that handle displaying properties of a plugin and reacting on changes of the ui when the user
/// wants to alter a property.
///
/// It is higly recommended not to implement this trait for your plugin by yourself. Instead use the provided derive macro called Plugin.
///
/// # Example
///
/// ```
/// #[derive(Plugin)]
/// pub struct Grid {
///     #[option(default=10, min=0, max=2000, label="Offset", description="The offset between two grid lines.")]
///     offset: u32,

///     #[option(default=2, min=0, max=100, label="Subdivisions", description="Subdivisions between offset")]
///     subdivisions: u8,
/// }
/// ```
///
/// The plugin will autogenerated the functions for you resulting in a consistent ui and proven functionality with standartized error handling.
/// 
#[allow(unused_variables)]
pub trait PluginWithOptions<Data, Modes>: AnyPlugin<Data>
where
    Data: Renderer + Default + 'static,
    Modes: Clone + PartialEq + Eq + Hash + 'static,
{
    fn view_options(&self, _context: &Context<App<Data, Modes>>) -> Html {
        html! {}
    }

    /// Called each time a property is updated. Use it to message the change or apply it to the plugin directly.
    fn update_property(&mut self, _property: &str, _value: Box<dyn Any>) {}

    fn identifier() -> &'static str
    where
        Self: Sized;

    fn enabled(&self) -> bool;

    /// Handles enabling a plugin by pressing its shortcut and is therefore executed even if the plugin is currently disabled. 
    fn __internal_key_up(&mut self, key: &str, special_keys: &Vec<SpecialKey>, _data: &mut Data) {}
}

#[allow(unused_variables)]
pub trait Plugin<Data>
where
    Data: Renderer,
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
    /// * `data` - The data hold by the editor
    fn mouse_up(&mut self, _mouse_pos: Coordinate<f64>, _button: u32, _data: &mut Data) {}

    /// React to a key held down on a keyboard.  
    /// 
    /// * 'key' the value of the pressed key. [See here for more informations](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    /// * `data` - The data hold by the editor
    fn key_down(&mut self, key: &str, _data: &mut Data) {}

    /// React to a key released on a keyboard.  
    /// 
    /// * 'key' the value of the released key. [See here for more informations](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    /// * `data` - The data hold by the editor
    fn key_up(&mut self, key: &str, _data: &mut Data) {}
}
