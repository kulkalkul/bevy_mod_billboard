use crate::pipeline::{
    extract_billboard, queue_billboard_bind_group, queue_billboard_texture,
    queue_billboard_view_bind_groups, ArrayImageCached, BillboardPipeline, BillboardTextPipeline,
    BillboardTexturePipeline, BillboardUniform, DrawBillboard, BillboardImageBindGroups,
};
use crate::text::update_billboard_text;
use crate::{
    BillboardMeshHandle, BillboardTextBounds, BillboardTexture, BILLBOARD_SHADER_HANDLE,
};
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::prelude::*;
use bevy::render::camera::CameraUpdateSystem;
use bevy::render::extract_component::UniformComponentPlugin;
use bevy::render::render_asset::RenderAssetPlugin;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedMeshPipelines;
use bevy::render::{RenderApp, RenderSet};

pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        shaders.set_untracked(
            BILLBOARD_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("shader/billboard.wgsl")),
        );

        app.add_asset::<BillboardTexture>()
            .add_plugin(UniformComponentPlugin::<BillboardUniform>::default())
            .add_plugin(RenderAssetPlugin::<BillboardTexture>::default())
            .register_type::<BillboardTextBounds>()
            .register_type::<BillboardMeshHandle>()
            .add_system(
                update_billboard_text
                    .in_base_set(CoreSet::PostUpdate)
                    .ambiguous_with(CameraUpdateSystem),
            );

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawBillboard>()
            .init_resource::<BillboardPipeline>()
            .init_resource::<BillboardTextPipeline>()
            .init_resource::<BillboardTexturePipeline>()
            .init_resource::<SpecializedMeshPipelines<BillboardPipeline>>()
            .init_resource::<SpecializedMeshPipelines<BillboardTextPipeline>>()
            .init_resource::<SpecializedMeshPipelines<BillboardTexturePipeline>>()
            .init_resource::<BillboardImageBindGroups>()
            .init_resource::<ArrayImageCached>()
            .add_system(extract_billboard.in_schedule(ExtractSchedule))
            .add_system(queue_billboard_bind_group.in_set(RenderSet::Queue))
            .add_system(queue_billboard_view_bind_groups.in_set(RenderSet::Queue))
            .add_system(queue_billboard_texture.in_set(RenderSet::Queue));
    }
}
