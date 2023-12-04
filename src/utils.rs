use bevy::{transform::components::{Transform, GlobalTransform}, math::{Mat4, Vec4}};

use crate::{pipeline::BillboardUniform, BillboardLockAxis};

pub fn compute_matrix_without_rotation(transform: &Transform) -> Mat4 {
    Mat4::from_cols(
        Mat4::IDENTITY.x_axis * transform.scale.x,
        Mat4::IDENTITY.y_axis * transform.scale.y,
        Mat4::IDENTITY.z_axis * transform.scale.z,
        Vec4::from((transform.translation, 1.0)),
    )
}

pub fn calculate_billboard_uniform(
    global_transform: &GlobalTransform,
    transform: &Transform,
    lock_axis: Option<&BillboardLockAxis>,
) -> BillboardUniform {
    let transform = if lock_axis.is_some() {
        global_transform.compute_matrix()
    } else {
        compute_matrix_without_rotation(transform)
    };

    BillboardUniform { transform }
}