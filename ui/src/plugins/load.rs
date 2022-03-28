use rust_editor::{
    keys,
    plugins::plugin::Plugin,
    store::Store,
    ui::{
        app::{EditorError, Shortkey},
        toolbar::ToolbarPosition,
    },
};
use rust_macro::editor_plugin;

use crate::map::map::Map;

#[editor_plugin(skip, specific_to=Map)]
pub struct Load {}

impl Plugin<Map> for Load {
    fn startup(&mut self, editor: &mut App<Map>) -> Result<(), EditorError> {
        editor.add_shortkey::<Load>(keys!["Control", "o"])?;

        let toolbar = editor.get_or_add_toolbar("primary.actions", ToolbarPosition::Left)?;
        toolbar.add_toggle_button(
            "open_in_browser",
            "load",
            "Load".to_string(),
            || false,
            || EditorMessages::ShortkeyPressed(keys!["Control", "o"]),
        )?;

        Ok(())
    }

    fn shortkey_pressed(&mut self, key: &Shortkey, _: &Context<App<Map>>, editor: &mut App<Map>) {
        if *key == keys!["Control", "o"] {
            if let Some(store) = Store::new("map_editor") {
                if let Some(data) = store.fetch_local_storage() {
                    editor.set_data(data);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_editor::keys;
    use rust_editor::plugins::plugin::Plugin;
    use rust_editor::ui::app::App;
    use rust_editor::ui::toolbar::ToolbarPosition;

    use crate::map::map::Map;
    use crate::plugins::load::Load;

    #[test]
    fn integration_startup_adds_shortcut() {
        let mut app = App::<Map>::default();

        let mut plugin = Load::default();
        plugin.startup(&mut app).unwrap();

        assert!(app.has_shortkey(keys!["Control", "o"]))
    }

    #[test]

    fn integration_startup_adds_toolbar_button() {
        let mut app = App::<Map>::default();

        let mut plugin = Load::default();
        plugin.startup(&mut app).unwrap();

        let toolbar = app
            .get_or_add_toolbar("primary.actions", ToolbarPosition::Left)
            .unwrap();

        assert!(toolbar.has_button("load"));
    }
}