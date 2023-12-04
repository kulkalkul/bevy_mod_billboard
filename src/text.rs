use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::Extract;
use bevy::sprite::Anchor;
use bevy::text::{
    BreakLineOn, FontAtlasSet, FontAtlasWarning, PositionedGlyph, Text2dBounds, TextLayoutInfo,
    TextPipeline, TextSettings, YAxisOrientation,
};
use bevy::utils::{hashbrown, HashMap, HashSet, PassHash};
use smallvec::SmallVec;
use crate::{BillboardDepth, BillboardLockAxis};
use crate::pipeline::BillboardUniform;

// Uses this as reference
// https://github.com/bevyengine/bevy/blob/v0.11.2/crates/bevy_text/src/text2d.rs

#[derive(Component, Copy, Clone, Debug, Reflect, Deref, Default)]
#[reflect(Component)]
pub struct BillboardTextBounds(pub Text2dBounds);

// TODO: Maybe use something like { Single(Group), Multi(SmallVec<[Group; 1]>) }, benchmark it
#[derive(Component, Clone, Debug, Deref, DerefMut, Default)]
pub struct BillboardTextHandles(pub SmallVec<[BillboardTextHandleGroup; 1]>);

#[derive(Clone, Debug, Default)]
pub struct BillboardTextHandleGroup {
    mesh: Handle<Mesh>,
    atlas: Handle<TextureAtlas>,
}

pub fn extract_billboard_text(
    mut commands: Commands,
    mut extracted_billboard_fonts: ResMut<ExtractedBillboards>,
    billboard_text_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &Transform,
            &TextLayoutInfo,
            &Anchor,
            &BillboardTextHandles,
            &BillboardDepth,
            Option<&BillboardLockAxis>,
        )>,
    >,
) {
    extracted_billboard_fonts.billboards.clear();

    let mut entities = Vec::new();

    for (
        entity,
        visibility,
        transform,
        info,
        anchor,
        handles,
        &depth,
        lock_axis,
    ) in &billboard_text_query {
        if !visibility.is_visible() {
            continue;
        }

        let text_anchor = -(anchor.as_vec() + 0.5);
        let alignment_translation = info.size * text_anchor;

        let matrix = transform.compute_matrix();

        for handle_group in handles.iter() {
            extracted_billboard_fonts.billboards.insert(entity, ExtractedBillboardText {
                transform: matrix,
                alignment_translation,
                mesh: handle_group.mesh.id(),
                texture: handle_group.atlas.id(),
                depth,
                lock_axis: lock_axis.copied(),
            });
        }
        entities.push((entity, BillboardUniform { transform: matrix }));
    }
    commands.insert_or_spawn_batch(entities);
}

pub fn update_billboard_text_layout(
    mut queue: Local<HashSet<Entity>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    fonts: Res<Assets<Font>>,
    text_settings: Res<TextSettings>,
    mut font_atlas_warning: ResMut<FontAtlasWarning>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut font_atlas_set_storage: ResMut<Assets<FontAtlasSet>>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut text_query: Query<(
        Entity,
        Ref<Text>,
        Ref<BillboardTextBounds>,
        &mut TextLayoutInfo,
        &mut BillboardTextHandles,
    )>,
) {
    const SCALE_FACTOR: f64 = 1.0;

    for (
        entity,
        text,
        bounds,
        mut text_layout_info,
        mut billboard_text_handles,
    ) in &mut text_query {
        if text.is_changed() || bounds.is_changed() || queue.remove(&entity) {
            let text_bounds = Vec2::new(
                if text.linebreak_behavior == BreakLineOn::NoWrap {
                    f32::INFINITY
                } else {
                    bounds.size.x
                },
                bounds.size.y,
            );

            let info = match text_pipeline.queue_text(
                &fonts,
                &text.sections,
                SCALE_FACTOR,
                text.alignment,
                text.linebreak_behavior,
                text_bounds,
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
            
            let length = info.glyphs.len();
            let mut atlases = HashMap::new();

            for glyph in &info.glyphs {
                // TODO: Maybe with clever caching, could be possible to get rid of or_insert_with,
                // TODO: though I don't know how much of a gain it would be. Just keeping this as a note.
                let entry = atlases
                    .entry(glyph.atlas_info.texture_atlas.clone_weak())
                    .or_insert_with(|| {
                        (
                            Vec::with_capacity(length),
                            texture_atlases
                                .get(&glyph.atlas_info.texture_atlas)
                                .expect("Atlas should exist"),
                        )
                    });

                entry.0.push(glyph.clone());
            }

            billboard_text_handles.clear();

            for (handle, (glyphs, atlas)) in atlases {
                let mut positions = Vec::with_capacity(info.glyphs.len() * 4);
                let mut uvs = Vec::with_capacity(info.glyphs.len() * 4);
                let mut colors = Vec::with_capacity(info.glyphs.len() * 4);
                let mut indices = Vec::with_capacity(info.glyphs.len() * 6);

                let mut color = Color::WHITE.as_linear_rgba_f32();
                let mut current_section = usize::MAX;

                for PositionedGlyph {
                    position,
                    size,
                    atlas_info,
                    section_index,
                    ..
                } in glyphs
                {
                    let index = positions.len() as u32;

                    let half_size = size / 2.0;
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

                    if section_index != current_section {
                        color = text.sections[section_index]
                            .style
                            .color
                            .as_linear_rgba_f32();
                        current_section = section_index;
                    }

                    colors.extend([color, color, color, color]);

                    indices.extend([index, index + 2, index + 1, index, index + 3, index + 2]);
                }

                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

                mesh.set_indices(Some(Indices::U32(indices)));

                billboard_text_handles.push(BillboardTextHandleGroup {
                    mesh: meshes.add(mesh),
                    atlas: handle,
                });
            }

            *text_layout_info = info;
        }
    }
}

#[derive(Resource, Default)]
pub struct ExtractedBillboards {
    pub billboards: hashbrown::HashMap<Entity, ExtractedBillboardText, PassHash>,
}

pub struct ExtractedBillboardText {
    pub alignment_translation: Vec2,
    pub mesh: HandleId,
    pub texture: HandleId,
    pub depth: BillboardDepth,
    pub lock_axis: Option<BillboardLockAxis>,
    pub transform: Mat4,
}