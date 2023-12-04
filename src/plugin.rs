use crate::pipeline::{
    queue_billboard_bind_group, queue_billboard_texture,
    queue_billboard_view_bind_groups, BillboardPipeline,
    DrawBillboard, BillboardUniform, BillboardImageBindGroups,
};
use crate::text::{extract_billboard_text, update_billboard_text_layout};
use crate::texture::extract_billboard_texture;
use crate::{
    BillboardTextBounds, BILLBOARD_SHADER_HANDLE, BillboardMesh, BillboardTexture,
};
use bevy::prelude::*;
use bevy::render::camera::CameraUpdateSystem;
use bevy::render::extract_component::UniformComponentPlugin;
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

        app
            .add_plugins(UniformComponentPlugin::<BillboardUniform>::default())
            .register_type::<BillboardMesh>()
            .register_type::<BillboardTexture>()
            .register_type::<BillboardTextBounds>()
            .add_systems(
                PostUpdate,
                update_billboard_text_layout.ambiguous_with(CameraUpdateSystem),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawBillboard>()
            .init_resource::<BillboardPipeline>()
            .init_resource::<SpecializedMeshPipelines<BillboardPipeline>>()
            .init_resource::<BillboardImageBindGroups>()
            .add_systems(ExtractSchedule, (extract_billboard_text, extract_billboard_texture))
            .add_systems(Render, queue_billboard_bind_group.in_set(RenderSet::Queue))
            .add_systems(
                Render,
                queue_billboard_view_bind_groups.in_set(RenderSet::Queue),
            )
            .add_systems(Render, queue_billboard_texture.in_set(RenderSet::Queue));
    }
}
