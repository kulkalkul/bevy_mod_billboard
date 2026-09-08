#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod pipeline;
pub mod plugin;
pub mod text;
pub mod texture;
mod utils;

use crate::text::{BillboardTextBounds, BillboardTextHandles};
use bevy::camera::visibility::{add_visibility_class, VisibilityClass};
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::sprite::Anchor;
use bevy::text::{LetterSpacing, LineHeight, TextSection};

/// Marker component for a billboarded texture.
///
/// Additionally insert a [`BillboardMesh`] to function.
#[derive(Clone, Component, Reflect)]
#[reflect(Component)]
#[require(Billboard, Transform, Visibility)]
pub struct BillboardTexture(pub Handle<Image>);

/// Marker component for billboarded text.
///
/// Optionally insert [`TextSpan`] children to render separate sections.
///
/// # Warning
///
/// This component is incompatible with Bevy's `Text` and `Text2d`!
/// Bevy will attempt to render the `Text` in the UI and `Text2d` as 2D
/// text, corrupting the internal `TextLayoutInfo` used by billboarding.
///
/// If you are not using `TextSpan` children, set the `String` field of
/// this struct.
#[derive(Clone, Component, Default)]
#[require(
    Billboard,
    BillboardTextBounds,
    BillboardTextHandles,
    TextLayout,
    TextFont,
    TextColor,
    LineHeight,
    LetterSpacing,
    Anchor,
    Transform,
    Visibility
)]
pub struct BillboardText(pub String);

impl BillboardText {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TextSection for BillboardText {
    fn get_text(&self) -> &str {
        self.0.as_str()
    }
    fn get_text_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl From<&str> for BillboardText {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for BillboardText {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
struct BillboardTextNeedsRerender;

#[derive(Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BillboardMesh(pub Handle<Mesh>);

#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct BillboardDepth(pub bool);

impl Default for BillboardDepth {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Default, Clone, Copy, Component, ExtractComponent, Debug, Reflect)]
#[require(BillboardDepth, VisibilityClass)]
#[component(on_add = add_visibility_class::<Billboard>)]
pub struct Billboard;

#[derive(Default, Clone, Copy, Component, Debug, Reflect)]
pub struct BillboardLockAxis {
    pub y_axis: bool,
    pub rotation: bool,
}

impl BillboardLockAxis {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_lock_y(y_axis: bool) -> Self {
        Self {
            y_axis,
            ..default()
        }
    }

    pub fn from_lock_rotation(rotation: bool) -> Self {
        Self {
            rotation,
            ..default()
        }
    }

    pub fn with_lock_y(mut self, y_axis: bool) -> Self {
        self.y_axis = y_axis;
        self
    }

    pub fn with_lock_rotation(mut self, rotation: bool) -> Self {
        self.rotation = rotation;
        self
    }
}

pub mod prelude {
    pub use crate::{
        plugin::BillboardPlugin, text::BillboardTextBounds, BillboardDepth, BillboardLockAxis,
        BillboardMesh, BillboardText, BillboardTexture,
    };
}
