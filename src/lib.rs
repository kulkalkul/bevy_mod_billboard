mod pipeline;
mod plugin;
mod text;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;

pub use crate::pipeline::{BillboardMeshHandle, BillboardTexture};
pub use crate::plugin::BillboardPlugin;
pub use crate::text::{BillboardTextBounds, BillboardTextSize};

pub(self) const BILLBOARD_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 12823766040132746076);

pub(self) const ATTRIBUTE_TEXTURE_ARRAY_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureArrayIndex", 584807746, VertexFormat::Sint32);

#[derive(Bundle, Default)]
pub struct BillboardTextureBundle {
    pub texture: Handle<BillboardTexture>,
    pub mesh: BillboardMeshHandle,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Default)]
pub struct BillboardTextBundle {
    pub text: Text,
    pub text_size: BillboardTextSize,
    pub text_bounds: BillboardTextBounds,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
