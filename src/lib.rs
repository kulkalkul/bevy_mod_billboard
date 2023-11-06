pub mod pipeline;
pub mod plugin;
pub mod text;

use bevy::prelude::*;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;
use bevy::sprite::Anchor;
use crate::pipeline::{BillboardMeshHandle, BillboardTexture};
use crate::text::BillboardTextBounds;

pub(self) const BILLBOARD_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(12823766040132746076);

pub(self) const ATTRIBUTE_TEXTURE_ARRAY_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureArrayIndex", 584807746, VertexFormat::Sint32);

#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct BillboardDepth(pub bool);

impl Default for BillboardDepth {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Default, Clone, Copy, Component, Debug, Reflect)]
pub struct BillboardLockAxis {
    pub y_axis: bool,
    pub rotation: bool,
}

#[derive(Bundle, Default)]
pub struct BillboardLockAxisBundle<T: Bundle> {
    pub billboard_bundle: T,
    pub lock_axis: BillboardLockAxis,
}

#[derive(Bundle, Default)]
pub struct BillboardTextureBundle {
    pub texture: Handle<BillboardTexture>,
    pub mesh: BillboardMeshHandle,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub billboard_depth: BillboardDepth,
}

#[derive(Bundle, Default)]
pub struct BillboardTextBundle {
    pub text: Text,
    pub text_bounds: BillboardTextBounds,
    pub text_anchor: Anchor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub billboard_depth: BillboardDepth,
}

pub mod prelude {
    pub use crate::{
        pipeline::{BillboardMeshHandle, BillboardTexture},
        plugin::BillboardPlugin,
        text::BillboardTextBounds,
        BillboardTextBundle,
        BillboardTextureBundle,
    };
}
