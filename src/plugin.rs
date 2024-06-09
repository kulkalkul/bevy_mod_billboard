use crate::pipeline::{
    prepare_billboard_bind_group, prepare_billboard_view_bind_groups, queue_billboard_texture,
    BillboardImageBindGroups, BillboardPipeline, BillboardUniform, DrawBillboard,
};
use crate::text::{extract_billboard_text, update_billboard_text_layout, BillboardTextHandles};
use crate::texture::extract_billboard_texture;
use crate::{
    BillboardMeshHandle, BillboardTextBounds, BillboardTextureHandle, BILLBOARD_SHADER_HANDLE,
};
use bevy::prelude::*;
use bevy::render::camera::CameraUpdateSystem;
use bevy::render::extract_component::UniformComponentPlugin;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedMeshPipelines;
use bevy::render::view::check_visibility;
use bevy::render::view::VisibilitySystems::CheckVisibility;
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

        app.add_plugins(UniformComponentPlugin::<BillboardUniform>::default())
            .register_type::<BillboardMeshHandle>()
            .register_type::<BillboardTextureHandle>()
            .register_type::<BillboardTextBounds>()
            .add_systems(
                PostUpdate,
                (
                    update_billboard_text_layout.ambiguous_with(CameraUpdateSystem),
                    check_visibility::<With<BillboardMeshHandle>>.in_set(CheckVisibility),
                    check_visibility::<With<BillboardTextHandles>>.in_set(CheckVisibility),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawBillboard>()
            .init_resource::<BillboardPipeline>()
            .init_resource::<SpecializedMeshPipelines<BillboardPipeline>>()
            .init_resource::<BillboardImageBindGroups>()
            .add_systems(
                ExtractSchedule,
                (extract_billboard_text, extract_billboard_texture),
            )
            .add_systems(Render, queue_billboard_texture.in_set(RenderSet::Queue))
            .add_systems(
                Render,
                prepare_billboard_bind_group.in_set(RenderSet::PrepareBindGroups),
            )
            .add_systems(
                Render,
                prepare_billboard_view_bind_groups.in_set(RenderSet::PrepareBindGroups),
            );
    }
}
