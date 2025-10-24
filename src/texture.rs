use bevy::{
    camera::visibility::ViewVisibility,
    ecs::system::{Commands, Local, Query},
    render::{sync_world::RenderEntity, Extract},
    transform::components::{GlobalTransform, Transform},
};

use crate::{
    pipeline::{RenderBillboardImage, RenderBillboardMesh},
    text::RenderBillboard,
    utils::calculate_billboard_uniform,
    BillboardDepth, BillboardLockAxis, BillboardMesh, BillboardTexture,
};

pub fn extract_billboard_texture(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    billboard_text_query: Extract<
        Query<(
            &RenderEntity,
            &ViewVisibility,
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
        render_entity,
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
            render_entity.id(),
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
    commands.insert_batch(batch);
}
