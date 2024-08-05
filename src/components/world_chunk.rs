use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand::{Rng, RngCore};

use crate::constant::chunk::{SIZE_X, SIZE_Y, TOTAL_SIZE};
use crate::constant::pixels::base_pixel::Pixel;
use crate::constant::pixels::{sand_pixel, space_pixel};
use crate::constant::world::{Direction, NUM_DIRECTIONS};
use crate::resources::chunk_map::ChunkMap;

#[derive(Component)]
pub struct WorldChunk {
    pub position: IVec2,
    pub pixel_data: Vec<u32>,
    pub last_updated: Vec<u8>,
    pub texture_handle: Handle<Image>,
    pub last_update_value: u8,
}

impl WorldChunk {
    pub fn new(position: IVec2, textures: &mut ResMut<Assets<Image>>) -> Self {
        let size = Extent3d {
            width: SIZE_X as u32,
            height: SIZE_Y as u32,
            depth_or_array_layers: 1,
        };

        // Initialize pixel_data with red color
        let default_colour = space_pixel::SpacePixel::get_random_colour();
        let pixel_data: Vec<u32> = vec![default_colour; TOTAL_SIZE as usize];

        let image = Image::new_fill(
            size,
            TextureDimension::D2,
            bytemuck::cast_slice(&pixel_data),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        let texture_handle = textures.add(image);

        WorldChunk {
            position,
            pixel_data: pixel_data,
            last_updated: vec![0; TOTAL_SIZE as usize],
            texture_handle,
            last_update_value: 0,
        }
    }

    pub fn update_texture(&self, images: &mut Assets<Image>) {
        if let Some(texture) = images.get_mut(&self.texture_handle) {
            // // Create a new vector to hold the u8 data
            let mut u8_data = Vec::with_capacity(self.pixel_data.len() * 4);

            // Convert each u32 value to four u8 values and push them to the new vector
            for &pixel in &self.pixel_data {
                u8_data.push((pixel >> 24) as u8); // Red
                u8_data.push((pixel >> 16) as u8); // Green
                u8_data.push((pixel >> 8) as u8); // Blue
                u8_data.push(pixel as u8); // Alpha
            }
            // println!("u8_data length: {}", u8_data.len());
            texture.data = u8_data;

            // // Update the texture with the latest pixel data
            // TODO: (James) Why does this not work? it is same size, same data same bit structure
            // texture.data = bytemuck::cast_slice(&self.pixel_data).to_vec() as Vec<u8>;
        }
    }

    pub fn update(&mut self) {
        self.last_update_value = self.last_update_value.wrapping_add(1);

        // TODO: (James) Should probably pass in a random number generator
        let mut rng = rand::thread_rng();
        for i in 0..SIZE_X as usize {
            // Use random, give 5% chance to set the pixel to white
            if rng.gen_range(0..100) < 5 {
                self.pixel_data[i] = sand_pixel::SandPixel::get_random_colour();
            }
        }

        // Update Pixels
        for i in 0..TOTAL_SIZE as usize {
            if self.pixel_data[i] != 0 {
                if self.last_updated[i] == self.last_update_value {
                    continue;
                }

                let _x = i % SIZE_X as usize;
                let _y = i / SIZE_X as usize;

                let downIndex = i + SIZE_X as usize;

                // Move the pixel down
                if _y < SIZE_Y as usize - 1 {
                    // confirm the pixel below it isn't black
                    if self.pixel_data[downIndex] != 0 {
                        continue;
                    }
                    self.pixel_data[downIndex] = self.pixel_data[i];
                    self.pixel_data[i] = 0x00000000;

                    self.last_updated[downIndex] = self.last_update_value;
                }
            }
        }
    }
}
