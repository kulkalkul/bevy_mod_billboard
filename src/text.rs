use crate::{BillboardMeshHandle, BillboardTexture, ATTRIBUTE_TEXTURE_ARRAY_INDEX};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};
use bevy::sprite::Anchor;
use bevy::text::{
    FontAtlasSet, FontAtlasWarning, PositionedGlyph, TextLayoutInfo, TextPipeline, TextSettings,
    YAxisOrientation,
};
use bevy::utils::{HashMap, HashSet};

// Uses these as reference
// https://github.com/bevyengine/bevy/blob/5718a7e74cc9b93d1d0bed9123548222123151b3/crates/bevy_text/src/text2d.rs
// https://github.com/bevyengine/bevy/blob/f749e734e798733dcaa13f0ce403dc3e1c00943a/crates/bevy_text/src/text3d.rs

// This is duplicate of Tex2dBounds; not sure if I should simply use Text2dBounds. Though, in that
// case, newtype over Text might be needed for proper querying, so default text renderer doesn't
// clash with this one.

#[derive(Component, Copy, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct BillboardTextBounds {
    pub size: Vec2,
}

impl Default for BillboardTextBounds {
    #[inline]
    fn default() -> Self {
        Self::UNBOUNDED
    }
}

impl BillboardTextBounds {
    pub const UNBOUNDED: Self = Self {
        size: Vec2::splat(f32::INFINITY),
    };
}


pub fn update_billboard_text(
    mut commands: Commands,
    mut queue: Local<HashSet<Entity>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    fonts: Res<Assets<Font>>,
    text_settings: Res<TextSettings>,
    mut font_atlas_warning: ResMut<FontAtlasWarning>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut billboard_textures: ResMut<Assets<BillboardTexture>>,
    mut font_atlas_set_storage: ResMut<Assets<FontAtlasSet>>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut text_query: Query<(
        Entity,
        Ref<Text>,
        Ref<BillboardTextBounds>,
        Option<&mut TextLayoutInfo>,
        &Anchor,
    )>,
) {
    for (entity,text, bounds, text_layout_info, anchor) in &mut text_query {
        if text.is_changed() || bounds.is_changed() || queue.remove(&entity) {
            let info = match text_pipeline.queue_text(
                &fonts,
                &text.sections,
                1.0,
                text.alignment,
                text.linebreak_behavior,
                bounds.size,
                &mut font_atlas_set_storage,
                &mut texture_atlases,
                &mut images,
                text_settings.as_ref(),
                &mut font_atlas_warning,
                YAxisOrientation::BottomToTop,
            ) {
                Err(TextError::NoSuchFont) => {
                    error!("Missing font (could still be loading)");
                    queue.insert(entity);
                    continue;
                }
                Err(err @ TextError::FailedToAddGlyph(_)) => {
                    panic!("Fatal error when processing text: {err}.");
                }
                Ok(info) => info,
            };

            let text_anchor = -(anchor.as_vec() + 0.5);
            let alignment_translation = info.size * text_anchor;

            let text_mesh_and_texture = build_text_mesh_and_texture(
                &text.sections,
                &mut texture_atlases,
                &mut images,
                &info,
                alignment_translation,
            );

            let (mesh, atlas_texture_handles, array_texture_handle) = match text_mesh_and_texture {
                None => {
                    commands
                        .entity(entity)
                        .insert(billboard_textures.add(BillboardTexture::Empty));
                    return;
                }
                Some(tuple) => tuple,
            };

            match text_layout_info {
                Some(mut t) => {
                    *t = info;
                    commands.entity(entity).insert((
                        BillboardMeshHandle(meshes.add(mesh)),
                        billboard_textures.add(BillboardTexture::Array {
                            array_handle: array_texture_handle,
                            atlas_handles: atlas_texture_handles,
                        }),
                    ));
                }
                None => {
                    commands.entity(entity).insert((
                        info,
                        BillboardMeshHandle(meshes.add(mesh)),
                        billboard_textures.add(BillboardTexture::Array {
                            array_handle: array_texture_handle,
                            atlas_handles: atlas_texture_handles,
                        }),
                    ));
                }
            }
        }
    }
}

fn build_text_mesh_and_texture(
    sections: &[TextSection],
    texture_atlases: &mut Assets<TextureAtlas>,
    images: &mut Assets<Image>,
    info: &TextLayoutInfo,
    alignment_translation: Vec2,
) -> Option<(Mesh, Vec<Handle<Image>>, Handle<Image>)> {
    let length = info.glyphs.len();
    let mut atlases = HashMap::new();

    for glyph in &info.glyphs {
        let entry = atlases
            .entry(glyph.atlas_info.texture_atlas.clone_weak())
            .or_insert_with(|| {
                (
                    Vec::with_capacity(length),
                    texture_atlases
                        .get(&glyph.atlas_info.texture_atlas)
                        .expect("Atlas not found."),
                )
            });
        entry.0.push(glyph.clone());
    }

    let mut positions = Vec::with_capacity(info.glyphs.len() * 4);
    let mut uvs = Vec::with_capacity(info.glyphs.len() * 4);
    let mut colors = Vec::with_capacity(info.glyphs.len() * 4);
    let mut texture_array_indices = Vec::with_capacity(info.glyphs.len() * 4);
    let mut indices = Vec::with_capacity(info.glyphs.len() * 6);

    let mut texture_atlas_handles = Vec::with_capacity(atlases.len());

    let mut biggest_size = Vec2::ZERO;

    for (atlas_index, (glyphs, atlas)) in atlases.values().enumerate() {
        for PositionedGlyph {
            position,
            size,
            atlas_info,
            section_index,
            ..
        } in glyphs {
            let index = positions.len() as u32;

            let position = *position + alignment_translation;

            let half_size = *size / 2.0;
            let top_left = position - half_size;
            let bottom_right = position + half_size;

            positions.extend([
                [top_left.x, top_left.y, 0.0],
                [top_left.x, bottom_right.y, 0.0],
                [bottom_right.x, bottom_right.y, 0.0],
                [bottom_right.x, top_left.y, 0.0],
            ]);

            let Rect { min, max } = atlas.textures[atlas_info.glyph_index];
            let min = min / atlas.size;
            let max = max / atlas.size;

            uvs.extend([
                [min.x, max.y],
                [min.x, min.y],
                [max.x, min.y],
                [max.x, max.y],
            ]);

            let color = sections[*section_index].style.color.as_linear_rgba_f32();
            colors.extend([color, color, color, color]);

            texture_array_indices.extend([
                atlas_index as i32,
                atlas_index as i32,
                atlas_index as i32,
                atlas_index as i32,
            ]);

            indices.extend([index, index + 2, index + 1, index, index + 3, index + 2]);
        }

        texture_atlas_handles.push(atlas.texture.clone_weak());

        let image = images.get_mut(&atlas.texture).unwrap();

        image.texture_descriptor.usage = TextureUsages::COPY_SRC | TextureUsages::COPY_DST;

        if atlas.size.cmpgt(biggest_size) != BVec2::FALSE {
            biggest_size = atlas.size;
        }
    }

    if positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(ATTRIBUTE_TEXTURE_ARRAY_INDEX, texture_array_indices);
    mesh.set_indices(Some(Indices::U32(indices)));

    let width = biggest_size.x.ceil() as u32;
    let height = biggest_size.y.ceil() as u32;
    let empty_buffer = vec![0; (width * height * texture_atlas_handles.len() as u32 * 4) as usize];

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: texture_atlas_handles.len() as u32,
        },
        TextureDimension::D2,
        empty_buffer,
        TextureFormat::Rgba8UnormSrgb,
    );

    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::D2Array),
        ..default()
    });

    let array_texture_handle = images.add(image);

    Some((mesh, texture_atlas_handles, array_texture_handle))
}
