pub struct SandPixel;

use super::base_pixel::{PixelType, MAX_COLOUR_COUNT};
use crate::constant::pixels::base_pixel::Pixel;

impl Pixel for SandPixel {
    const PIXEL_TYPE: PixelType = PixelType::SandPixel;
    const PIXEL_INDEX: u8 = 1;
    const IS_UPDATABLE: bool = true;
    const PIXEL_NAME: &'static str = "SandPixel";
    const TYPE_COLOURS: [u32; MAX_COLOUR_COUNT as usize] =
        [0xFFFFFFFF, 0x00FF00FF, 0xFF0000FF, 0x0000FFFF];
    const TYPE_COLOURS_COUNT: usize = 4;
}

#[test]
fn test_get_random_colour_distribution() {
    let mut counts = [0; SandPixel::TYPE_COLOURS_COUNT];
    let colours = SandPixel::TYPE_COLOURS;
    for _ in 0..1000 {
        let colour = SandPixel::get_random_colour();
        let mut found = false;
        for (i, &c) in colours.iter().enumerate() {
            if colour == c {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            panic!("Invalid colour");
        }
    }
    for &count in counts.iter() {
        assert!(count > 0);
    }
    for (i, &count) in counts.iter().enumerate() {
        println!("Colour {:X} has count {}", colours[i], count);
    }
}
