use std::time::Duration;

use crate::components::world_chunk::WorldChunk;
use bevy::prelude::*;
use crate::resources::chunk_map::ChunkMap;

pub fn sand_step(chunk_map: Res<ChunkMap>) {
    for (_, mut chunk) in chunk_map.get_iter_mut() {
        // transform.translation.y -= 10.0 * fixed_delta_time;
        // chunk.update_position(transform.translation);

        chunk.update();
    }
}

pub fn update_textures(
    chunk_map: Res<ChunkMap>,
    mut images: ResMut<Assets<Image>>,
) {
    for (_, chunk) in chunk_map.get_iter() {
        // Apply your physics logic here
        // transform.translation.y -= 10.0 * fixed_delta_time;
        // chunk.update_position(transform.translation);

        chunk.update_texture(&mut images);
    }
}
