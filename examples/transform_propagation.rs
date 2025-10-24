use bevy::color::palettes;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BillboardPlugin)
        .add_systems(Startup, (setup_billboard, setup_scene))
        .add_systems(Update, move_cube)
        .run();
}

#[derive(Component)]
pub struct ParentCube;

fn setup_billboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");

    commands
        .spawn((
            ParentCube,
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::Srgba(palettes::css::GRAY))),
            Transform::from_translation(Vec3::new(0.0, -2.0, 1.0)),
        ))
        .with_child((
            BillboardText::new("parented text"),
            TextFont::from(fira_sans_regular_handle).with_font_size(60.0),
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::splat(0.0085)),
            TextLayout::new_with_justify(Justify::Center),
        ));
}

// Important bits are above, the code below is for camera, reference cube and rotation

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn move_cube(
    mut parent_cube: Query<&mut Transform, With<ParentCube>>,
    mut accumulated: Local<f32>,
    mut direction: Local<bool>,
    time: Res<Time>,
) {
    let mut parent_cube = parent_cube.single_mut().unwrap();

    let direction_vec = if *direction { Vec3::Z } else { Vec3::NEG_Z };

    parent_cube.translation += time.delta_secs() * direction_vec;
    *accumulated += time.delta_secs();

    if *accumulated >= 2.0 {
        *direction = !*direction;
        *accumulated -= 2.0
    }
}
