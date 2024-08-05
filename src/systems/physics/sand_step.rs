use std::time::Duration;

use crate::components::world_chunk::WorldChunk;
use bevy::prelude::*;

pub fn sand_step(mut query: Query<(&mut Transform, &mut WorldChunk)>) {
    for (mut transform, mut chunk) in query.iter_mut() {
        // transform.translation.y -= 10.0 * fixed_delta_time;
        // chunk.update_position(transform.translation);

        chunk.update();
    }
}

pub fn update_textures(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut Transform, &mut WorldChunk)>,
) {
    for (mut transform, mut chunk) in query.iter_mut() {
        // Apply your physics logic here
        // transform.translation.y -= 10.0 * fixed_delta_time;
        // chunk.update_position(transform.translation);

        chunk.update_texture(&mut images);
    }
}
