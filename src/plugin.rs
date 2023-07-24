use crate::pipeline::{
    extract_billboard, queue_billboard_bind_group, queue_billboard_texture,
    queue_billboard_view_bind_groups, ArrayImageCached, BillboardPipeline, BillboardTextPipeline,
    BillboardTexturePipeline, BillboardUniform, DrawBillboard, BillboardImageBindGroups,
};
use crate::text::update_billboard_text;
use crate::{
    BillboardMeshHandle, BillboardTextBounds, BillboardTexture, BILLBOARD_SHADER_HANDLE,
};
use bevy::prelude::*;
use bevy::render::camera::CameraUpdateSystem;
use bevy::render::extract_component::UniformComponentPlugin;
use bevy::render::render_asset::RenderAssetPlugin;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedMeshPipelines;
use bevy::render::{RenderApp, RenderSet};
use bevy::{asset::load_internal_asset, core_pipeline::core_3d::Transparent3d, render::Render};

pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            BILLBOARD_SHADER_HANDLE,
            "shader/billboard.wgsl",
            Shader::from_wgsl
        );

        app.add_asset::<BillboardTexture>()
            .add_plugins(UniformComponentPlugin::<BillboardUniform>::default())
            .add_plugins(RenderAssetPlugin::<BillboardTexture>::default())
            .register_type::<BillboardTextBounds>()
            .register_type::<BillboardMeshHandle>()
            .add_systems(
                PostUpdate,
                update_billboard_text.ambiguous_with(CameraUpdateSystem),
            );
    }

    fn finish(&self, app: &mut App) {
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
            .add_systems(ExtractSchedule, extract_billboard)
            .add_systems(Render, queue_billboard_bind_group.in_set(RenderSet::Queue))
            .add_systems(
                Render,
                queue_billboard_view_bind_groups.in_set(RenderSet::Queue),
            )
            .add_systems(Render, queue_billboard_texture.in_set(RenderSet::Queue));
    }
}
