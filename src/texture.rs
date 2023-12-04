use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Local, Query},
    },
    render::{view::ViewVisibility, Extract},
    transform::components::{GlobalTransform, Transform},
};

use crate::{
    pipeline::{RenderBillboardImage, RenderBillboardMesh},
    text::RenderBillboard,
    utils::calculate_billboard_uniform,
    BillboardDepth, BillboardLockAxis, BillboardMeshHandle, BillboardTextureHandle,
};

pub fn extract_billboard_texture(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    billboard_text_query: Extract<
        Query<(
            Entity,
            &ViewVisibility,
            &GlobalTransform,
            &Transform,
            &BillboardMeshHandle,
            &BillboardTextureHandle,
            &BillboardDepth,
            Option<&BillboardLockAxis>,
        )>,
    >,
) {
    let mut batch = Vec::with_capacity(*previous_len);

    for (
        entity,
        visibility,
        global_transform,
        transform,
        billboard_mesh,
        billboard_texture,
        &depth,
        lock_axis,
    ) in &billboard_text_query
    {
        if !visibility.get() {
            continue;
        }

        let uniform = calculate_billboard_uniform(global_transform, transform, lock_axis);

        batch.push((
            entity,
            (
                uniform,
                RenderBillboardMesh {
                    id: billboard_mesh.0.id(),
                },
                RenderBillboardImage {
                    id: billboard_texture.0.id(),
                },
                RenderBillboard {
                    depth,
                    lock_axis: lock_axis.copied(),
                },
            ),
        ));
    }

    *previous_len = batch.len();
    commands.insert_or_spawn_batch(batch);
}
