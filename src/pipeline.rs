use crate::{ATTRIBUTE_TEXTURE_ARRAY_INDEX, BILLBOARD_SHADER_HANDLE};
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::system::lifetimeless::{Read, SQuery, SRes};
use bevy::ecs::system::{SystemParamItem, SystemState};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout, PrimitiveTopology};
use bevy::render::render_asset::{PrepareAssetError, RenderAssets};
use bevy::render::render_phase::{
    DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline,
    TrackedRenderPass,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent, BlendFactor,
    BlendOperation, BlendState, BufferBindingType, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, CompareFunction, DepthStencilState, Extent3d, FragmentState,
    FrontFace, ImageCopyTexture, MultisampleState, Origin3d, PipelineCache, PolygonMode,
    PrimitiveState, RenderPipelineDescriptor, SamplerBindingType, ShaderStages, ShaderType,
    SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines, TextureAspect,
    TextureFormat, TextureSampleType, TextureViewDimension, VertexState,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::texture::{BevyDefault, GpuImage};
use bevy::render::view::{
    ExtractedView, ViewUniform, ViewUniformOffset, ViewUniforms, VisibleEntities,
};
use bevy::render::Extract;
use bevy::utils::{HashMap, HashSet};

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "4977f56e-6ad1-4fe2-a8b3-a757036eeaac"]
pub enum BillboardTexture {
    Single(Handle<Image>),
    Array {
        array_handle: Handle<Image>,
        atlas_handles: Vec<Handle<Image>>,
    },
    Empty,
}

impl BillboardTexture {
    fn handle(&self) -> Option<&Handle<Image>> {
        match self {
            BillboardTexture::Single(handle) => Some(handle),
            BillboardTexture::Array { array_handle, .. } => Some(array_handle),
            BillboardTexture::Empty => None,
        }
    }
}

impl Default for BillboardTexture {
    fn default() -> Self {
        Self::Single(default())
    }
}

impl bevy::render::render_asset::RenderAsset for BillboardTexture {
    type ExtractedAsset = BillboardTexture;
    type PreparedAsset = BillboardTexture;
    type Param = ();

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        _param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        Ok(extracted_asset)
    }
}

enum BillboardTextureType<'image> {
    Single(Handle<Image>, &'image GpuImage),
    Array(Handle<Image>, &'image GpuImage),
}

#[derive(Default, Clone, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct BillboardMeshHandle(pub Handle<Mesh>);

impl From<Handle<Mesh>> for BillboardMeshHandle {
    fn from(handle: Handle<Mesh>) -> Self {
        Self(handle)
    }
}

#[derive(Component, Clone, ShaderType)]
pub struct BillboardUniform {
    transform: Mat4,
}

#[derive(Resource)]
pub struct BillboardBindGroup {
    value: BindGroup,
}

#[derive(Component)]
pub struct BillboardViewBindGroup {
    value: BindGroup,
}

#[derive(Resource, Default)]
pub struct ImageBindGroups {
    values: HashMap<Handle<Image>, BindGroup>,
}

#[derive(Resource, Default)]
pub struct ArrayImageCached {
    cached: HashSet<Handle<Image>>,
}

impl ArrayImageCached {
    fn cached_copy<'a>(
        &'a mut self,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
        billboard_texture: &BillboardTexture,
        render_images: &'a RenderAssets<Image>,
    ) -> Option<BillboardTextureType<'a>> {
        match billboard_texture {
            BillboardTexture::Empty => None,
            BillboardTexture::Single(handle) => {
                let Some(image) = render_images.get(handle) else { return None; };

                Some(BillboardTextureType::Single(handle.clone_weak(), image))
            }
            BillboardTexture::Array {
                array_handle,
                atlas_handles,
            } => {
                let Some(array_image) = render_images.get(array_handle) else { return None; };
                if self.cached.contains(array_handle) {
                    return Some(BillboardTextureType::Array(
                        array_handle.clone_weak(),
                        array_image,
                    ));
                }

                let mut command_encoder =
                    render_device.create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("create_texture_array_from_font_atlas"),
                    });

                for (index, handle) in atlas_handles.iter().enumerate() {
                    let Some(image) = render_images.get(handle) else { return None; };

                    command_encoder.copy_texture_to_texture(
                        ImageCopyTexture {
                            texture: &image.texture,
                            mip_level: 0,
                            origin: Origin3d { x: 0, y: 0, z: 0 },
                            aspect: TextureAspect::All,
                        },
                        ImageCopyTexture {
                            texture: &array_image.texture,
                            mip_level: 0,
                            origin: Origin3d {
                                x: 0,
                                y: 0,
                                z: index as u32,
                            },
                            aspect: TextureAspect::All,
                        },
                        Extent3d {
                            width: image.size.x as u32,
                            height: image.size.y as u32,
                            depth_or_array_layers: 1,
                        },
                    );
                }

                self.cached.insert(array_handle.clone_weak());

                render_queue.submit(vec![command_encoder.finish()]);
                Some(BillboardTextureType::Array(
                    array_handle.clone_weak(),
                    array_image,
                ))
            }
        }
    }
}

// Reference:
// https://github.com/bevyengine/bevy/blob/release-0.9.1/crates/bevy_sprite/src/mesh2d/mesh.rs#L282
bitflags::bitflags! {
    #[repr(transparent)]
    // NOTE: Apparently quadro drivers support up to 64x MSAA.
    // MSAA uses the highest 3 bits for the MSAA log2(sample count) to support up to 128x MSAA.
    pub struct BillboardPipelineKey: u32 {
        const TEXT               = 0;
        const TEXTURE            = (1 << 0);
        const MSAA_RESERVED_BITS = Self::MSAA_MASK_BITS << Self::MSAA_SHIFT_BITS;
    }
}

impl BillboardPipelineKey {
    const MSAA_MASK_BITS: u32 = 0b111;
    const MSAA_SHIFT_BITS: u32 = 32 - Self::MSAA_MASK_BITS.count_ones();

    pub fn from_msaa_samples(msaa_samples: u32) -> Self {
        let msaa_bits =
            (msaa_samples.trailing_zeros() & Self::MSAA_MASK_BITS) << Self::MSAA_SHIFT_BITS;
        Self::from_bits(msaa_bits).unwrap()
    }
    pub fn msaa_samples(&self) -> u32 {
        1 << ((self.bits >> Self::MSAA_SHIFT_BITS) & Self::MSAA_MASK_BITS)
    }
}

#[derive(Resource, Clone)]
pub struct BillboardPipeline {
    view_layout: BindGroupLayout,
    billboard_layout: BindGroupLayout,
}

impl FromWorld for BillboardPipeline {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(Res<RenderDevice>,)> = SystemState::new(world);

        let (render_device,) = system_state.get(world);

        let view_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("billboard_view_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            }],
        });

        let billboard_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("billboard_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(BillboardUniform::min_size()),
                },
                count: None,
            }],
        });
        Self {
            view_layout,
            billboard_layout,
        }
    }
}

impl SpecializedMeshPipeline for BillboardPipeline {
    type Key = BillboardPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        const DEF_VERTEX_COLOR: &str = "VERTEX_COLOR";
        const DEF_VERTEX_TEXTURE_ARRAY: &str = "VERTEX_TEXTURE_ARRAY";

        let mut shader_defs = Vec::with_capacity(4);
        let mut attributes = Vec::with_capacity(4);

        attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
        attributes.push(Mesh::ATTRIBUTE_UV_0.at_shader_location(1));

        if layout.contains(Mesh::ATTRIBUTE_COLOR) {
            shader_defs.push(DEF_VERTEX_COLOR.to_string());
            attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(2));
        }
        if layout.contains(ATTRIBUTE_TEXTURE_ARRAY_INDEX) {
            shader_defs.push(DEF_VERTEX_TEXTURE_ARRAY.to_string());
            attributes.push(ATTRIBUTE_TEXTURE_ARRAY_INDEX.at_shader_location(3));
        }

        let vertex_buffer_layout = layout.get_layout(&attributes)?;

        Ok(RenderPipelineDescriptor {
            label: Some("billboard_pipeline".into()),
            layout: Some(vec![
                self.view_layout.clone(),
                self.billboard_layout.clone(),
            ]),
            vertex: VertexState {
                shader: BILLBOARD_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                buffers: vec![vertex_buffer_layout],
                shader_defs: shader_defs.clone(),
            },
            fragment: Some(FragmentState {
                shader: BILLBOARD_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "fragment".into(),
                shader_defs,
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Greater,
                stencil: default(),
                bias: default(),
            }),
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        })
    }
}

fn texture_layout_descriptor(
    render_device: &RenderDevice,
    view_dimension: TextureViewDimension,
) -> BindGroupLayout {
    render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("billboard_texture_layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

#[derive(Resource, Clone)]
pub struct BillboardTextPipeline {
    billboard_pipeline: BillboardPipeline,
    texture_layout: BindGroupLayout,
}

impl FromWorld for BillboardTextPipeline {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(Res<RenderDevice>, Res<BillboardPipeline>)> =
            SystemState::new(world);

        let (render_device, billboard_pipeline) = system_state.get(world);
        let texture_layout =
            texture_layout_descriptor(&render_device, TextureViewDimension::D2Array);

        Self {
            billboard_pipeline: billboard_pipeline.clone(),
            texture_layout,
        }
    }
}

impl SpecializedMeshPipeline for BillboardTextPipeline {
    type Key = BillboardPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        self.billboard_pipeline
            .specialize(key, layout)
            .map(|mut descriptor| {
                descriptor
                    .layout
                    .as_mut()
                    .unwrap()
                    .push(self.texture_layout.clone());
                descriptor
            })
    }
}

#[derive(Resource, Clone)]
pub struct BillboardTexturePipeline {
    billboard_pipeline: BillboardPipeline,
    texture_layout: BindGroupLayout,
}

impl FromWorld for BillboardTexturePipeline {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(Res<RenderDevice>, Res<BillboardPipeline>)> =
            SystemState::new(world);

        let (render_device, billboard_pipeline) = system_state.get(world);
        let texture_layout = texture_layout_descriptor(&render_device, TextureViewDimension::D2);

        Self {
            billboard_pipeline: billboard_pipeline.clone(),
            texture_layout,
        }
    }
}

impl SpecializedMeshPipeline for BillboardTexturePipeline {
    type Key = BillboardPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        self.billboard_pipeline
            .specialize(key, layout)
            .map(|mut descriptor| {
                descriptor
                    .layout
                    .as_mut()
                    .unwrap()
                    .push(self.texture_layout.clone());
                descriptor
            })
    }
}

pub fn extract_billboard(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &GlobalTransform,
            &Handle<BillboardTexture>,
            &BillboardMeshHandle,
        )>,
    >,
) {
    let mut values = Vec::with_capacity(*previous_len);

    for (entity, visibility, transform, billboard_texture_handle, billboard_mesh_handle) in
        query.iter()
    {
        if !visibility.is_visible() {
            continue;
        }

        // TODO: Maybe reset rotation elsewhere
        let (scale, _, translation) = transform.to_scale_rotation_translation();
        let transform = Transform {
            translation,
            scale,
            ..default()
        }
        .compute_matrix();

        values.push((
            entity,
            (
                billboard_texture_handle.clone_weak(),
                BillboardMeshHandle(billboard_mesh_handle.0.clone_weak()),
                BillboardUniform { transform },
            ),
        ));
    }

    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

pub fn queue_billboard_view_bind_groups(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    billboard_pipeline: Res<BillboardPipeline>,
    view_uniforms: Res<ViewUniforms>,
    views: Query<Entity, With<ExtractedView>>,
) {
    let Some(binding) = view_uniforms.uniforms.binding() else { return; };

    for entity in views.iter() {
        commands.entity(entity).insert(BillboardViewBindGroup {
            value: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("billboard_view_bind_group"),
                layout: &billboard_pipeline.view_layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: binding.clone(),
                }],
            }),
        });
    }
}

pub fn queue_billboard_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    billboard_pipeline: Res<BillboardPipeline>,
    billboard_uniforms: Res<ComponentUniforms<BillboardUniform>>,
) {
    let Some(binding) = billboard_uniforms.uniforms().binding() else { return; };

    commands.insert_resource(BillboardBindGroup {
        value: render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("billboard_bind_group"),
            layout: &billboard_pipeline.billboard_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: binding,
            }],
        }),
    });
}

pub fn queue_billboard_texture(
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<Transparent3d>,
    )>,
    mut text_pipelines: ResMut<SpecializedMeshPipelines<BillboardTextPipeline>>,
    mut texture_pipelines: ResMut<SpecializedMeshPipelines<BillboardTexturePipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut image_bind_groups: ResMut<ImageBindGroups>,
    mut array_image_cached: ResMut<ArrayImageCached>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    transparent_draw_functions: Res<DrawFunctions<Transparent3d>>,
    billboard_text_pipeline: Res<BillboardTextPipeline>,
    billboard_texture_pipeline: Res<BillboardTexturePipeline>,
    msaa: Res<Msaa>,
    render_images: Res<RenderAssets<Image>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    billboard_textures: Res<RenderAssets<BillboardTexture>>,
    billboards: Query<(
        &Handle<BillboardTexture>,
        &BillboardUniform,
        &BillboardMeshHandle,
    )>,
) {
    for (view, visible_entities, mut transparent_phase) in &mut views {
        let draw_transparent_billboard = transparent_draw_functions
            .read()
            .get_id::<DrawBillboard>()
            .unwrap();

        let rangefinder = view.rangefinder3d();

        for visible_entity in &visible_entities.entities {
            let Ok((
                       billboard_texture_handle,
                       billboard_uniform,
                       billboard_mesh_handle,
                   )) = billboards.get(*visible_entity) else { continue; };
            let Some(mesh) = render_meshes.get(&billboard_mesh_handle.0) else { continue; };
            let Some(billboard_texture) = billboard_textures.get(billboard_texture_handle) else { continue; };
            let Some(billboard_type) = array_image_cached.cached_copy(
                &render_device,
                &render_queue,
                billboard_texture,
                &render_images,
            ) else { continue; };

            let key = BillboardPipelineKey::from_msaa_samples(msaa.samples);

            let (array_handle, array_image, pipeline_id, texture_layout) = match billboard_type {
                BillboardTextureType::Single(array_handle, array_image) => (
                    array_handle,
                    array_image,
                    texture_pipelines.specialize(
                        &mut pipeline_cache,
                        &billboard_texture_pipeline,
                        key | BillboardPipelineKey::TEXTURE,
                        &mesh.layout,
                    ),
                    &billboard_texture_pipeline.texture_layout,
                ),
                BillboardTextureType::Array(array_handle, array_image) => (
                    array_handle,
                    array_image,
                    text_pipelines.specialize(
                        &mut pipeline_cache,
                        &billboard_text_pipeline,
                        key | BillboardPipelineKey::TEXT,
                        &mesh.layout,
                    ),
                    &billboard_text_pipeline.texture_layout,
                ),
            };

            let pipeline_id = match pipeline_id {
                Ok(id) => id,
                Err(err) => {
                    error!("{err:?}");
                    continue;
                }
            };

            let distance = rangefinder.distance(&billboard_uniform.transform);

            image_bind_groups
                .values
                .entry(array_handle)
                .or_insert_with(|| {
                    render_device.create_bind_group(&BindGroupDescriptor {
                        label: Some("billboard_texture_bind_group"),
                        layout: texture_layout,
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: BindingResource::TextureView(&array_image.texture_view),
                            },
                            BindGroupEntry {
                                binding: 1,
                                resource: BindingResource::Sampler(&array_image.sampler),
                            },
                        ],
                    })
                });

            transparent_phase.add(Transparent3d {
                pipeline: pipeline_id,
                entity: *visible_entity,
                draw_function: draw_transparent_billboard,
                distance,
            });
        }
    }
}

pub struct SetBillboardViewBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetBillboardViewBindGroup<I> {
    type Param = SQuery<(Read<ViewUniformOffset>, Read<BillboardViewBindGroup>)>;

    #[inline]
    fn render<'w>(
        view: Entity,
        _item: Entity,
        view_query: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let (view_uniform, billboard_mesh_bind_group) = view_query.get_inner(view).unwrap();

        pass.set_bind_group(I, &billboard_mesh_bind_group.value, &[view_uniform.offset]);

        RenderCommandResult::Success
    }
}

pub struct SetBillboardBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetBillboardBindGroup<I> {
    type Param = (
        SRes<BillboardBindGroup>,
        SQuery<Read<DynamicUniformIndex<BillboardUniform>>>,
    );

    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (billboard_bind_group, billboard_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let billboard_index = billboard_query.get(item).unwrap();

        pass.set_bind_group(
            I,
            &billboard_bind_group.into_inner().value,
            &[billboard_index.index()],
        );

        RenderCommandResult::Success
    }
}

pub struct SetBillboardTextureBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetBillboardTextureBindGroup<I> {
    type Param = (
        SRes<ImageBindGroups>,
        SRes<RenderAssets<BillboardTexture>>,
        SQuery<Read<Handle<BillboardTexture>>>,
    );

    fn render<'w>(
        _view: Entity,
        item: Entity,
        (images, billboard_textures, query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let billboard_texture_handle = query.get(item).unwrap();
        let billboard_texture = billboard_textures.get(billboard_texture_handle).unwrap();

        match billboard_texture.handle() {
            None => RenderCommandResult::Failure,
            Some(handle) => {
                let bind_group = images.into_inner().values.get(handle).unwrap();

                pass.set_bind_group(I, bind_group, &[]);

                RenderCommandResult::Success
            }
        }
    }
}

pub struct DrawBillboardMesh;
impl EntityRenderCommand for DrawBillboardMesh {
    type Param = (SRes<RenderAssets<Mesh>>, SQuery<Read<BillboardMeshHandle>>);

    fn render<'w>(
        _view: Entity,
        item: Entity,
        (meshes, mesh_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let billboard_mesh_handle = mesh_query.get(item).unwrap();

        if let Some(gpu_mesh) = meshes.into_inner().get(&billboard_mesh_handle.0) {
            pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));

            match &gpu_mesh.buffer_info {
                GpuBufferInfo::Indexed {
                    buffer,
                    index_format,
                    count,
                } => {
                    pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                    pass.draw_indexed(0..*count, 0, 0..1);
                }
                GpuBufferInfo::NonIndexed { vertex_count } => {
                    pass.draw(0..*vertex_count, 0..1);
                }
            }

            RenderCommandResult::Success
        } else {
            RenderCommandResult::Failure
        }
    }
}

pub type DrawBillboard = (
    SetItemPipeline,
    SetBillboardViewBindGroup<0>,
    SetBillboardBindGroup<1>,
    SetBillboardTextureBindGroup<2>,
    DrawBillboardMesh,
);
