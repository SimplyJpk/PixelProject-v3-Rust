pub struct SpacePixel;

use super::base_pixel::{PixelType, MAX_COLOUR_COUNT};
use crate::constant::pixels::base_pixel::Pixel;

impl Pixel for SpacePixel {
    const PIXEL_TYPE: PixelType = PixelType::SpacePixel;
    const PIXEL_INDEX: u8 = 0;
    const IS_UPDATABLE: bool = false;
    const PIXEL_NAME: &'static str = "SpacePixel";
    const TYPE_COLOURS: [u32; MAX_COLOUR_COUNT as usize] =
        [0x00000000, 0x00000000, 0x00000000, 0x00000000];
    const TYPE_COLOURS_COUNT: usize = 1;
}
