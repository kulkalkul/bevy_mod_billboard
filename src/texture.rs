use bevy::{ecs::{system::{Commands, Local, Query}, entity::Entity}, render::{Extract, view::ComputedVisibility}, transform::components::{GlobalTransform, Transform}};

use crate::{utils::calculate_billboard_uniform, BillboardDepth, BillboardLockAxis, pipeline::{RenderBillboardMesh, RenderBillboardImage}, text::RenderBillboard, BillboardMesh, BillboardTexture};

pub fn extract_billboard_texture(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    billboard_text_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &GlobalTransform,
            &Transform,
            &BillboardMesh,
            &BillboardTexture,
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
    ) in &billboard_text_query {
        if !visibility.is_visible() {
            continue;
        }

        let uniform = calculate_billboard_uniform(global_transform, transform, lock_axis);

        batch.push((
            entity,
            (
                uniform,
                RenderBillboardMesh { id: billboard_mesh.0.id() },
                RenderBillboardImage { id: billboard_texture.0.id() },
                RenderBillboard {
                    depth,
                    lock_axis: lock_axis.copied(),
                }
            )
        ));
    }

    *previous_len = batch.len();
    commands.insert_or_spawn_batch(batch);
}