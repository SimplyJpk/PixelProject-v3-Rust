use std::time::Duration;

use crate::components::world_chunk::WorldChunk;
use bevy::prelude::*;
use crate::resources::chunk_map::ChunkMap;

pub fn sand_step(mut chunk_map: ResMut<ChunkMap>) {
    for (_, chunk) in chunk_map.chunks.iter_mut() {
        // transform.translation.y -= 10.0 * fixed_delta_time;
        // chunk.update_position(transform.translation);

        chunk.update();
    }
}

pub fn update_textures(
    mut chunk_map: ResMut<ChunkMap>,
    mut images: ResMut<Assets<Image>>,
) {
    for (_, chunk) in chunk_map.chunks.iter_mut() {
        // Apply your physics logic here
        // transform.translation.y -= 10.0 * fixed_delta_time;
        // chunk.update_position(transform.translation);

        chunk.update_texture(&mut images);
    }
}
