use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::world_chunk::WorldChunk;

#[derive(Default, Resource)]
pub struct ChunkMap {
    pub chunks: HashMap<IVec2, WorldChunk>,
}
