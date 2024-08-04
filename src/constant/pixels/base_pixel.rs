use rand::Rng;

pub const MAX_COLOUR_COUNT: u32 = 4;

pub enum PixelType {
    SpacePixel,
    SandPixel,
}

pub trait Pixel {
    const PIXEL_TYPE: PixelType;
    const PIXEL_INDEX: u8;
    const IS_UPDATABLE: bool;
    const PIXEL_NAME: &'static str;
    const TYPE_COLOURS: [u32; MAX_COLOUR_COUNT as usize];
    const TYPE_COLOURS_COUNT: usize = MAX_COLOUR_COUNT as usize;

    fn pixel_type() -> PixelType {
        Self::PIXEL_TYPE
    }

    fn pixel_index() -> u8 {
        Self::PIXEL_INDEX
    }

    fn is_updatable() -> bool {
        Self::IS_UPDATABLE
    }

    fn pixel_name() -> &'static str {
        Self::PIXEL_NAME
    }

    fn type_colours() -> &'static [u32; MAX_COLOUR_COUNT as usize] {
        &Self::TYPE_COLOURS
    }

    fn colour_count() -> usize {
        Self::TYPE_COLOURS_COUNT as usize
    }

    fn get_random_colour() -> u32 {
        if Self::TYPE_COLOURS_COUNT == 1 {
            return Self::TYPE_COLOURS[0];
        }
        let randomIndex = rand::thread_rng().gen_range(0..Self::TYPE_COLOURS_COUNT);
        println!(
            "Random index for pixel {} is {}",
            Self::PIXEL_NAME,
            randomIndex
        );
        return Self::TYPE_COLOURS[randomIndex as usize];
    }

    fn get_new_pixel() -> u32 {
        Self::PIXEL_INDEX as u32
    }
}
