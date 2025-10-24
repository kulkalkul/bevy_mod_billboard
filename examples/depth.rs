use bevy::color::palettes;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BillboardPlugin)
        .add_systems(Startup, (setup_billboard, setup_scene))
        .add_systems(Update, rotate_camera)
        .run();
}

const TEXT_SCALE: Vec3 = Vec3::splat(0.0085);

fn setup_billboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = TextFont::from(asset_server.load("FiraSans-Regular.ttf")).with_font_size(60.0);

    commands.spawn((
        BillboardText::new("depth enabled"),
        text_font.clone(),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 0.5, 0.0).with_scale(TEXT_SCALE),
    ));

    commands.spawn((
        BillboardText::new("depth disabled"),
        BillboardDepth(false),
        text_font.clone(),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -0.5, 0.0).with_scale(TEXT_SCALE),
    ));
}

// Important bits are above, the code below is for camera, reference cube and rotation

#[derive(Component)]
#[require(Transform)]
pub struct CameraHolder;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(CameraHolder).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(5., 0., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::Srgba(palettes::css::BEIGE))),
        Transform::from_xyz(1., 0., 0.),
    ));
}

fn rotate_camera(mut camera: Query<&mut Transform, With<CameraHolder>>, time: Res<Time>) {
    let mut camera = camera.single_mut().unwrap();

    camera.rotate_y(time.delta_secs());
}
