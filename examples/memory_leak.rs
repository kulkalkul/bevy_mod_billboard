use std::mem::size_of_val;

use bevy::{ecs::entity::Entities, prelude::*, render::RenderApp, sprite::ImageBindGroups};
use bevy_mod_billboard::{
    pipeline::{
        ArrayImageCached, BillboardPipeline, BillboardTextPipeline, BillboardTexturePipeline,
    },
    BillboardPlugin, BillboardTextBundle,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(BillboardPlugin)
        .add_startup_system(spawn_camera)
        // By running the systems in this order we can actually see our billboard to verify that it's working
        .add_systems(
            (
                despawn_billboard,
                spawn_billboard,
                debug_main_app_memory_use,
            )
                .chain(),
        );

    app.sub_app_mut(RenderApp)
        .add_system(debug_render_app_memory_use);

    app.run();
}

/// Avoid constantly reloading the font
#[derive(Resource)]
struct FontHandle(Handle<Font>);

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5., 0., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(FontHandle(asset_server.load("FiraSans-Regular.ttf")));
}

fn spawn_billboard(mut commands: Commands, font: Res<FontHandle>) {
    commands.spawn(BillboardTextBundle {
        transform: Transform::from_scale(Vec3::splat(0.0085)),
        text: Text::from_sections([
            TextSection {
                value: "IMPORTANT".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: font.0.clone_weak(),
                    color: Color::ORANGE,
                },
            },
            TextSection {
                value: " text".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: font.0.clone_weak(),
                    color: Color::WHITE,
                },
            },
        ])
        .with_alignment(TextAlignment::Center),
        ..default()
    });
}

fn despawn_billboard(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn debug_main_app_memory_use(entities: &Entities) {
    info!("{} main world entities", entities.len());
}

fn debug_render_app_memory_use(
    entities: &Entities,
    image_bind_groups: Res<ImageBindGroups>,
    array_image_cached: Res<ArrayImageCached>,
    billboard_pipeline: Res<BillboardPipeline>,
    billboard_text_pipeline: Res<BillboardTextPipeline>,
    billboard_texture_pipeline: Res<BillboardTexturePipeline>,
) {
    info!("{} render entities", entities.len());
    info!("ImageBindGroups size: {}", size_of_val(&image_bind_groups));
    info!(
        "ArrayImageCached size: {}",
        size_of_val(&array_image_cached)
    );
    info!(
        "BillboardPipeline size: {}",
        size_of_val(&billboard_pipeline)
    );
    info!(
        "BillboardTextPipeline size: {}",
        size_of_val(&billboard_text_pipeline)
    );
    info!(
        "BillboardTexturePipeline size: {}",
        size_of_val(&billboard_texture_pipeline)
    );
}
