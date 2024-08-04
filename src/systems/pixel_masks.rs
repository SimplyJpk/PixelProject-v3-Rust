// src/systems/pixel_masks.rs

pub trait PixelType {
    const BITS: u32;
    const COUNT: u8;
    const DEPTH: u8;
    const MASK: u32 = Self::BITS << Self::DEPTH;

    fn get_mask() -> u32 {
        MASK
    }

    fn get_max_value() -> u32 {
        (1 << Self::COUNT) - 1
    }

    fn get_value(value: u32) -> u8 {
        ((value & Self::BITS) >> Self::DEPTH) as u8
    }
}

// Masks
pub struct PixelTypeIndex;
pub struct PixelLifetime;
pub struct PixelBehaviour;
pub struct PixelLight;

impl PixelType for PixelTypeIndex {
    const BITS: u32 = 0b0000_0000_0000_0000_0000_0000_0001_1111;
    const COUNT: u8 = 5;
    const DEPTH: u8 = 0;
}

impl PixelType for PixelLifetime {
    const BITS: u32 = 0b1111_1100_0000_0000_0000_0000_0000_0000;
    const COUNT: u8 = 6;
    const DEPTH: u8 = 26 + 1;
}

impl PixelType for PixelBehaviour {
    const BITS: u32 = 0b0000_0011_1100_0000_0000_0000_0000_0000;
    const COUNT: u8 = 4;
    const DEPTH: u8 = 22 + 1;
}

impl PixelType for PixelLight {
    const BITS: u32 = 0b0000_0000_0000_0000_0000_0000_1110_0000;
    const COUNT: u8 = 3;
    const DEPTH: u8 = 5 + 1;
}

// Remaining usable bits
pub const PIXEL_REMAINING_BITS: u32 = 0b0000_0000_0011_1111_1111_1111_0000_0000;
