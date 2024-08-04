use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand::{Rng, RngCore};

use crate::constant::chunk::{SIZE_X, SIZE_Y, TOTAL_SIZE};
use crate::constant::pixels::base_pixel::Pixel;
use crate::constant::pixels::{sand_pixel, space_pixel};
use crate::constant::world::NUM_DIRECTIONS;

#[derive(Component)]
pub struct WorldChunk {
    pub position: IVec2,
    // pub neighbour_chunks: [Option<Entity>; NUM_DIRECTIONS as usize],
    pub pixel_data: Vec<u32>,
    pub last_updated: Vec<u8>,
    pub texture_handle: Handle<Image>,
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
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        let texture_handle = textures.add(image);

        WorldChunk {
            position,
            pixel_data: vec![0; TOTAL_SIZE as usize],
            last_updated: vec![0; TOTAL_SIZE as usize],
            texture_handle,
        }
    }

    pub fn update_texture(&self, images: &mut Assets<Image>) {
        if let Some(texture) = images.get_mut(&self.texture_handle) {
            // Update the texture with the latest pixel data
            texture.data = bytemuck::cast_slice(&self.pixel_data).to_vec();
        }
    }

    pub fn update(&mut self, images: &mut Assets<Image>) {
        let mut rng = rand::thread_rng();
        for i in 0..SIZE_X as usize {
            // Use random, give 5% chance to set the pixel to white
            if rng.gen_range(0..100) < 5 {
                self.pixel_data[i] = sand_pixel::SandPixel::get_random_colour();
            }
        }

        // loop through all pixels again, if the pixel isn't black then move it down
        for i in 0..TOTAL_SIZE as usize {
            if self.pixel_data[i] != 0 {
                if rng.gen_range(0..100) > 5 {
                    continue;
                }
                let x = i % SIZE_X as usize;
                let y = i / SIZE_X as usize;

                // Move the pixel down
                if y < SIZE_Y as usize - 1 {
                    // confirm the pixel below it isn't black
                    if self.pixel_data[i + SIZE_X as usize] != 0 {
                        continue;
                    }
                    self.pixel_data[i + SIZE_X as usize] = self.pixel_data[i];
                    self.pixel_data[i] = 0x00000000;
                }
            }
        }

        self.update_texture(images);
    }
}
