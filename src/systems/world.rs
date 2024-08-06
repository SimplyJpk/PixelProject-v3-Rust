use std::collections::HashMap;

use crate::components::world_chunk::WorldChunk;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::Image;
use bytemuck::{cast_slice, Pod, Zeroable};

use crate::constant::chunk::{SIZE_X, SIZE_Y, TOTAL_SIZE};
use crate::constant::world::{Direction, NUM_DIRECTIONS};
use crate::resources::chunk_map::ChunkMap;

pub fn setup_world(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut chunk_map: ResMut<ChunkMap>
) {
    // Calculate spacing based on texture size plus 1-2 pixels
    let spacing_x = SIZE_X as f32 + 2.0;
    let spacing_y = SIZE_Y as f32 + 2.0;

    for x in 0..3 {
        for y in 0..3 {
            let chunk = WorldChunk::new(IVec2::new(x, y), &mut textures);

            commands
                .spawn(SpriteBundle {
                    texture: chunk.texture_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * spacing_x,
                        y as f32 * spacing_y,
                        0.0,
                    )),
                    ..Default::default()
                });

            chunk_map.chunks.insert(chunk.position, chunk);
        }
    }
}
