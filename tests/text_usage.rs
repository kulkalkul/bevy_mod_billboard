use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

// TODO: Doesn't work yet. WinitPlugin is incompatible with integration tests.
#[test]
fn text_binding_compatible_with_ui() {
    fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
        let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");

        let style = TextStyle {
            font: fira_sans_regular_handle.clone(),
            font_size: 60.0,
            color: Color::WHITE,
        };

        commands.spawn(Camera3dBundle::default());

        commands
            .spawn(BillboardTextBundle {
                text: Text::from_section("a", style.clone()),
                ..default()
            });

        commands.spawn(TextBundle::from_section("b", style));
    }

    App::new()
        .add_plugins((
            DefaultPlugins,
            BillboardPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .run();
}