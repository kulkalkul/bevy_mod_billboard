use crate::pipeline::{
    init_billboard_pipeline, prepare_billboard_bind_group, prepare_billboard_view_bind_groups,
    queue_billboard_texture, BillboardImageBindGroups, BillboardPipeline, BillboardUniform,
    DrawBillboard,
};
use crate::text::{
    detect_billboard_text_color_change, extract_billboard_text, update_billboard_text_layout,
    BillboardTextHandles,
};
use crate::texture::extract_billboard_texture;
use crate::{prelude::*, Billboard};
use bevy::camera::CameraUpdateSystems;
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponentPlugin, UniformComponentPlugin};
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedMeshPipelines;
use bevy::render::{RenderApp, RenderStartup, RenderSystems};
use bevy::shader::load_shader_library;
use bevy::text::detect_text_needs_rerender;
use bevy::{core_pipeline::core_3d::Transparent3d, render::Render};

pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "shader/billboard.wgsl");

        app.add_plugins(UniformComponentPlugin::<BillboardUniform>::default())
            .add_plugins(ExtractComponentPlugin::<Billboard>::default())
            .register_type::<BillboardMesh>()
            .register_type::<BillboardTexture>()
            .register_type::<BillboardTextBounds>()
            .register_type::<BillboardTextHandles>()
            .add_systems(
                PostUpdate,
                (
                    (
                        detect_text_needs_rerender::<BillboardText>,
                        detect_billboard_text_color_change,
                    ),
                    update_billboard_text_layout,
                )
                    .chain()
                    .ambiguous_with(CameraUpdateSystems),
            );

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawBillboard>()
            .init_resource::<SpecializedMeshPipelines<BillboardPipeline>>()
            .init_resource::<BillboardImageBindGroups>()
            .add_systems(RenderStartup, init_billboard_pipeline)
            .add_systems(
                ExtractSchedule,
                (extract_billboard_text, extract_billboard_texture),
            )
            .add_systems(Render, queue_billboard_texture.in_set(RenderSystems::Queue))
            .add_systems(
                Render,
                prepare_billboard_bind_group.in_set(RenderSystems::PrepareBindGroups),
            )
            .add_systems(
                Render,
                prepare_billboard_view_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            );
    }
}
