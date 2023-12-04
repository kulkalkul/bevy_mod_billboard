pub mod pipeline;
pub mod plugin;
pub mod text;
pub mod texture;
mod utils;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::text::{BillboardTextBounds, BillboardTextHandles};

pub(self) const BILLBOARD_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(12823766040132746076);

#[derive(Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct BillboardMesh(pub Handle<Mesh>);

#[derive(Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct BillboardTexture(pub Handle<Image>);

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
    pub texture: BillboardTexture,
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
    pub billboard_text_handles: BillboardTextHandles,
}

pub mod prelude {
    pub use crate::{
        BillboardTexture,
        BillboardMesh,
        BillboardTextBundle,
        BillboardTextureBundle,
        plugin::BillboardPlugin,
        text::BillboardTextBounds,
    };
}