use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::prelude::{IntoSystemConfigs, Mat4, Plugin, Res, ResMut, Resource};
use bevy::render::extract_component::{DynamicUniformIndex};
use bevy::render::render_resource::{DynamicUniformBuffer, ShaderType};
use bevy::render::{Render, RenderApp, RenderSet};
use bevy::render::renderer::{RenderDevice, RenderQueue};


#[derive(Clone, Copy, ShaderType, Component)]
pub struct BillboardUniform {
    pub(crate) transform: Mat4,
}

#[derive(Resource, Default)]
pub struct BillboardUniforms {
    pub uniforms: Vec<BillboardUniform>,
}

#[derive(Resource, Default)]
pub struct BillboardUniformsBuffer {
    uniforms: DynamicUniformBuffer<BillboardUniform>,
}

impl BillboardUniformsBuffer {
    #[inline]
    pub fn uniforms(&self) -> &DynamicUniformBuffer<BillboardUniform> {
        &self.uniforms
    }
}

#[derive(Resource, Default)]
pub struct BillboardUniformIndices {
    indices: Vec<DynamicUniformIndex<BillboardUniform>>,
}
pub struct BillboardUniformPlugin;

impl Plugin for BillboardUniformPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<BillboardUniforms>()
                .init_resource::<BillboardUniformsBuffer>()
                .init_resource::<BillboardUniformIndices>()
                .add_systems(
                    Render,
                    prepare_billboard_uniforms.in_set(RenderSet::Prepare),
                );
        }
    }
}

fn prepare_billboard_uniforms(
    mut billboard_uniforms: ResMut<BillboardUniforms>,
    mut billboard_uniforms_buffer: ResMut<BillboardUniformsBuffer>,
    mut billboard_uniform_indices: ResMut<BillboardUniformIndices>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    billboard_uniforms_buffer.uniforms.clear();
    billboard_uniform_indices.indices.clear();

    for uniform in billboard_uniforms.uniforms {

    }

    billboard_uniforms_buffer
        .uniforms
        .write_buffer(&render_device, &render_queue);
}