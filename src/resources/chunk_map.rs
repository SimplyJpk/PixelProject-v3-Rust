use std::collections::hash_map::Iter;
use bevy::prelude::*;
use std::collections::HashMap;
use std::iter::Map;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::components::world_chunk::WorldChunk;

#[derive(Default, Resource, Clone)]
pub struct ChunkMap {
    pub chunks: HashMap<IVec2, Arc<RwLock<WorldChunk>>>,
}

impl ChunkMap {
    /// Gets an iterable array of all chunk values
    pub fn get_iter(&self) -> impl Iterator<Item = (&IVec2, RwLockReadGuard<WorldChunk>)> {
        self.chunks.iter().map(|(pos, chunk)| {
            (pos, chunk.read().unwrap())
        })
    }

    /// Gets an iterable array of all chunk values
    pub fn get_iter_mut(&self) -> impl Iterator<Item = (&IVec2, RwLockWriteGuard<WorldChunk>)> {
        self.chunks.iter().map(|(pos, chunk)| {
            (pos, chunk.write().unwrap())
        })
    }

    pub fn get_chunk_mut(&self, center: IVec2) -> Option<RwLockWriteGuard<WorldChunk>> {
        let Some(mut chunk) = self.chunks.get(&center) else {
            return None
        };

        Some(chunk.write().unwrap())
    }

    pub fn get_chunk(&self, center: IVec2) -> Option<RwLockReadGuard<WorldChunk>> {
        let Some(chunk) = self.chunks.get(&center) else {
            return None
        };

        Some(chunk.read().unwrap())
    }

    /// Get `ChunkMap` of only surrounding neighbors for chunking at separate thread
    pub fn split_at_center(&self, center: IVec2) -> Self {
        let mut chunks = HashMap::new();
        let distance = 1;

        for x in (center.x - distance)..=(center.x + distance) {
            for y in (center.y - distance)..=(center.y + distance) {
                let pos = center + IVec2::new(x, y);
                if let Some(val) = self.chunks.get(&pos) {
                    chunks.insert(pos, val.clone());
                }
            }
        }

        ChunkMap {
            chunks
        }
    }
}