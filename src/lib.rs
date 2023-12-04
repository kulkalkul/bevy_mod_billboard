pub mod pipeline;
pub mod plugin;
pub mod text;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::Anchor;
use bevy::text::TextLayoutInfo;
use crate::text::{BillboardTextBounds, BillboardTextHandles};

pub(self) const BILLBOARD_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 12823766040132746076);

#[derive(Clone, Copy, ShaderType, Component)]
pub struct BillboardUniform {
    pub(crate) transform: Mat4,
}

#[derive(Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct BillboardMesh(pub Handle<Mesh>);

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
    pub mesh: BillboardMesh,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
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
    pub computed_visibility: ComputedVisibility,
    pub billboard_depth: BillboardDepth,
    pub billboard_text_handles: BillboardTextHandles,
    pub text_layout_info: TextLayoutInfo,
}

pub mod prelude {
    pub use crate::{
        pipeline::RenderBillboardMesh,
        plugin::BillboardPlugin,
        text::BillboardTextBounds,
        BillboardTextBundle,
        BillboardTextureBundle,
    };
}