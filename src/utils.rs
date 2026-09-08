use bevy::{
    math::Mat4,
    transform::components::{GlobalTransform, Transform},
};

use crate::{pipeline::BillboardUniform, BillboardLockAxis};

// TODO: Maybe add scale as uniform to shader and do this in shader?
pub fn compute_matrix_without_rotation(
    global_transform: &GlobalTransform,
    transform: &Transform,
) -> Mat4 {
    let global_matrix = global_transform.to_matrix();
    Mat4::from_cols(
        Mat4::IDENTITY.x_axis * transform.scale.x,
        Mat4::IDENTITY.y_axis * transform.scale.y,
        Mat4::IDENTITY.z_axis * transform.scale.z,
        global_matrix.w_axis,
    )
}

pub fn calculate_billboard_uniform(
    global_transform: &GlobalTransform,
    transform: &Transform,
    lock_axis: Option<&BillboardLockAxis>,
) -> BillboardUniform {
    let transform = if lock_axis.is_some() {
        global_transform.to_matrix()
    } else {
        compute_matrix_without_rotation(global_transform, transform)
    };

    BillboardUniform { transform }
}
