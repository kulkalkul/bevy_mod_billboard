use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

// TODO: Doesn't work yet. WinitPlugin is incompatible with integration tests.
#[test]
fn text_binding_compatible_with_ui() {
    fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
        let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");
        let text_font = TextFont::from(fira_sans_regular_handle).with_font_size(60.0);

        commands.spawn(Camera3d::default());

        commands.spawn((
            BillboardText::new("a"),
            text_font.clone(),
            TextColor::from(Color::WHITE),
        ));

        commands.spawn((Text::new("b"), text_font, TextColor::from(Color::WHITE)));
    }

    App::new()
        .add_plugins((DefaultPlugins, BillboardPlugin))
        .add_systems(Startup, setup_scene)
        .run();
}
