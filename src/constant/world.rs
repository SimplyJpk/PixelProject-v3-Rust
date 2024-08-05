pub const NUM_DIRECTIONS: u8 = 8;

// an Enum with 8 variants in compass-like order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub const ALL: [Direction; NUM_DIRECTIONS as usize] = [
        Direction::Up,
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
        Direction::DownLeft,
        Direction::Left,
        Direction::UpLeft,
    ];
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        Self::ALL.iter().copied()
    }
}

pub const DIRECTION_VECTORS: [(i8, i8); NUM_DIRECTIONS as usize] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub const fn direction_name(direction: Direction) -> &'static str {
    match direction {
        Direction::Up => "Up",
        Direction::UpRight => "UpRight",
        Direction::Right => "Right",
        Direction::DownRight => "DownRight",
        Direction::Down => "Down",
        Direction::DownLeft => "DownLeft",
        Direction::Left => "Left",
        Direction::UpLeft => "UpLeft",
    }
}

pub const fn direction_vector(direction: Direction) -> (i8, i8) {
    DIRECTION_VECTORS[direction as usize]
}

pub const DIRECTION_MASKS: [u8; NUM_DIRECTIONS as usize] = [
    1 << 0, // Up
    1 << 1, // UpRight
    1 << 2, // Right
    1 << 3, // DownRight
    1 << 4, // Down
    1 << 5, // DownLeft
    1 << 6, // Left
    1 << 7, // UpLeft
];

pub const fn direction_mask(direction: Direction) -> u8 {
    DIRECTION_MASKS[direction as usize]
}
