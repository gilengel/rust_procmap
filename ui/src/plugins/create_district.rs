use std::fmt;

use rust_macro::editor_plugin;

use crate::Map;
use geo::Coordinate;
use rust_editor::{
    actions::{Action, Redo, Undo},
    input::{keyboard::Key, mouse},
    plugin::{Plugin, PluginWithOptions},
    ui::app::{EditorError, Shortkey},
};
use uuid::Uuid;
use plugin_toolbar::toolbar::ToolbarPosition;

#[editor_plugin(specific_to=Map, execution=Exclusive)]
pub struct CreateDistrict {
    #[option(
        default = 500.,
        min = 0.,
        max = 100000.,
        label = "Minimum House Side",
        description = ""
    )]
    minimum_house_side: f64,

    //#[option(skip)]
    //seed: <ChaCha8Rng as SeedableRng>::Seed,
}

struct CreateDistrictAction {
    street: Uuid,
    //minimum_house_side: f64,
    //seed: <ChaCha8Rng as SeedableRng>::Seed,

    district: Option<Uuid>,
}

impl Redo<Map> for CreateDistrictAction {
    fn redo(&mut self, _map: &mut Map) {}
}

impl Undo<Map> for CreateDistrictAction {
    fn undo(&mut self, map: &mut Map) {
        if let Some(district) = self.district {
            map.remove_district(&district);
        }
    }
}

impl Action<Map> for CreateDistrictAction {}

impl fmt::Display for CreateDistrictAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[create_district] street={}", self.street)
    }
}

/*
impl CreateDistrictAction {
    pub fn new(
        street: Uuid,
        //minimum_house_side: f64,
        //seed: <ChaCha8Rng as SeedableRng>::Seed,
    ) -> Self {
        CreateDistrictAction {
            street,
            //minimum_house_side,
            //seed,
            district: None,
        }
    }
}
*/

impl Plugin<Map> for CreateDistrict {
    fn startup(&mut self, editor: &mut App<Map>) -> Result<(), EditorError> {
        editor.add_shortkey::<CreateDistrict>(vec![Key::Ctrl, Key::D])?;

        editor.plugin_mut(
            move |toolbar_plugin: &mut plugin_toolbar::ToolbarPlugin<Map>| {
                let toolbar = toolbar_plugin
                    .get_or_add_toolbar("primary.edit.modes.district", ToolbarPosition::Left)
                    .unwrap();

                let enabled = Rc::clone(&self.__enabled);
                toolbar
                    .add_toggle_button(
                        "maps_home_work",
                        "create_district",
                        "Create District".to_string(),
                        move || *enabled.as_ref().borrow(),
                        move || EditorMessages::ActivatePlugin(CreateDistrict::identifier()),
                    )
                    .unwrap();
            },
        );

        Ok(())
    }

    fn property_updated(&mut self, _: &str, editor: &mut App<Map>) {
        editor
            .data_mut()
            .districts_mut()
            .iter_mut()
            .for_each(|(_, x)| {
                x.minimum_house_side = self.minimum_house_side.clamp(20.0, 1000.0);
                x.update_houses();
            });
    }

    fn shortkey_pressed(&mut self, key: &Shortkey, ctx: &Context<App<Map>>, _: &mut App<Map>) {
        if *key == vec![Key::Ctrl, Key::D] {
            ctx.link()
                .send_message(EditorMessages::ActivatePlugin(CreateDistrict::identifier()));
        }
    }

    fn mouse_move(
        &mut self,
        _: Coordinate<f64>,
        _: Coordinate<f64>,
        _: mouse::Button,
        _: &mut App<Map>,
    ) -> bool {
        false
    }

    fn mouse_up(&mut self, _: Coordinate<f64>, _: mouse::Button, _: &mut App<Map>) -> bool {
        false
    }
}


/*
#[cfg(test)]
mod tests {
    use plugin_toolbar::toolbar::ToolbarPosition;
    use rust_editor::input::keyboard::Key;
    use rust_editor::plugin::Plugin;
    use rust_editor::ui::app::App;

    use crate::map::map::Map;
    use crate::plugins::create_district::CreateDistrict;

    #[test]
    fn integration_startup_adds_shortcut() {
        let mut app = App::<Map>::default();

        let mut plugin = CreateDistrict::default();
        plugin.startup(&mut app).unwrap();

        assert!(app.has_shortkey(vec![Key::Ctrl, Key::D]))
    }

    #[test]

    fn integration_startup_adds_toolbar_button() {
        let mut app = App::<Map>::default();

        let mut plugin = CreateDistrict::default();
        plugin.startup(&mut app).unwrap();

        let toolbar = app
            .get_or_add_toolbar("primary.edit.modes.district", ToolbarPosition::Left)
            .unwrap();

        assert!(toolbar.has_button("create_district"));
    }
}
*/
