use geo::Coordinate;
use rust_editor::{
    gizmo::Id,
    interactive_element::{InteractiveElement, InteractiveElementState},
    keys,
    plugins::plugin::{Plugin, PluginWithOptions},
    ui::{
        app::{EditorError, Shortkey},
        toolbar::ToolbarPosition,
    },
};
use rust_macro::editor_plugin;
use uuid::Uuid;

use crate::map::{district::District, map::Map};

#[editor_plugin(skip, specific_to=Map, execution=Exclusive)]
pub struct DeleteDistrict {
    #[option(skip)]
    hovered_district: Option<Uuid>,
}

impl Plugin<Map> for DeleteDistrict {
    fn startup(&mut self, editor: &mut App<Map>) -> Result<(), EditorError> {
        editor.add_shortkey::<DeleteDistrict>(keys!["Control", "f"])?;

        let toolbar =
            editor.get_or_add_toolbar("primary.edit.modes.district", ToolbarPosition::Left)?;

        let enabled = Rc::clone(&self.__enabled);
        toolbar.add_toggle_button(
            "delete_outline",
            "delete_district",
            "Delete District".to_string(),
            move || *enabled.as_ref().borrow(),
            move || EditorMessages::ActivatePlugin(DeleteDistrict::identifier()),
        )?;

        Ok(())
    }

    fn shortkey_pressed(&mut self, key: &Shortkey, ctx: &Context<App<Map>>, _: &mut App<Map>) {
        if *key == keys!["Control", "f"] {
            ctx.link()
                .send_message(EditorMessages::ActivatePlugin(DeleteDistrict::identifier()));
        }
    }

    fn mouse_move(
        &mut self,
        mouse_pos: Coordinate<f64>,
        _mouse_movement: Coordinate<f64>,
        editor: &mut App<Map>,
    ) {
        if let Some(old_hovered_district) = self.hovered_district {
            let old_hovered_district: &mut District = editor
                .data_mut()
                .district_mut(&old_hovered_district)
                .unwrap();
            old_hovered_district.set_state(InteractiveElementState::Normal);
        }

        if let Some(hovered_district) = editor.data().get_district_at_position(&mouse_pos) {
            let hovered_district: &mut District =
                editor.data_mut().district_mut(&hovered_district).unwrap();
            hovered_district.set_state(InteractiveElementState::Hover);
            self.hovered_district = Some(hovered_district.id());
        }
    }

    fn mouse_up(&mut self, mouse_pos: Coordinate<f64>, _button: u32, app: &mut App<Map>) {
        if let Some(hovered_district) = app.data().get_district_at_position(&mouse_pos) {
            app.data_mut().remove_district(&hovered_district);
            self.hovered_district = None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use rust_editor::keys;
    use rust_editor::plugins::plugin::Plugin;
    use rust_editor::ui::app::App;
    use rust_editor::ui::toolbar::ToolbarPosition;

    use crate::map::map::Map;
    use crate::plugins::delete_district::DeleteDistrict;

    #[test]
    fn integration_startup_adds_shortcut() {
        let mut app = App::<Map>::default();

        let mut delete_district_plugin = DeleteDistrict {
            hovered_district: None,
            __enabled: Rc::new(RefCell::new(true)),
            __execution_behaviour: rust_internal::PluginExecutionBehaviour::Exclusive,
        };
        delete_district_plugin.startup(&mut app).unwrap();

        assert!(app.has_shortkey(keys!["Control", "f"]))
    }

    #[test]

    fn integration_startup_adds_toolbar_button() {
        let mut app = App::<Map>::default();

        let mut delete_district_plugin = DeleteDistrict {
            hovered_district: None,
            __enabled: Rc::new(RefCell::new(true)),
            __execution_behaviour: rust_internal::PluginExecutionBehaviour::Exclusive,
        };
        delete_district_plugin.startup(&mut app).unwrap();

        let toolbar = app
            .get_or_add_toolbar("primary.edit.modes.district", ToolbarPosition::Left)
            .unwrap();

        assert!(toolbar.has_button("delete_district"));
    }
}