pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_C: u64 = FILE_A << 2;
pub const FILE_D: u64 = FILE_A << 3;
pub const FILE_E: u64 = FILE_A << 4;
pub const FILE_F: u64 = FILE_A << 5;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;

pub const RANK_1: u64 = 0xff;
pub const RANK_2: u64 = RANK_1 << (8 * 1);
pub const RANK_3: u64 = RANK_1 << (8 * 2);
pub const RANK_4: u64 = RANK_1 << (8 * 3);
pub const RANK_5: u64 = RANK_1 << (8 * 4);
pub const RANK_6: u64 = RANK_1 << (8 * 5);
pub const RANK_7: u64 = RANK_1 << (8 * 6);
pub const RANK_8: u64 = RANK_1 << (8 * 7);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

trait Bitboard {
    fn shift(&self, direction: Direction) -> Self;
}

impl Bitboard for u64 {
    fn shift(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self << 8,
            // if current is on an outer rim return 0 = dont wrap around the board
            Direction::UpRight => (self & !FILE_H) << 8 + 1,
            Direction::Right => (self & !FILE_H) << 1,
            Direction::DownRight => (self & !FILE_H) >> 8 - 1,
            Direction::Down => self >> 8,
            Direction::DownLeft => (self & !FILE_A) >> 8 + 1,
            Direction::Left => (self & !FILE_A) >> 1,
            Direction::UpLeft => (self & !FILE_A) << 8 - 1,
        }
    }
}

pub fn generate_attack_masks_rook() -> [u64; 64] {
    let mut attack_masks = [0; 64];

    for i in 0..64 {
        let mut mask = 0;
        mask |= RANK_1 << (i / 8) * 8;
        mask |= FILE_A << i % 8;

        attack_masks[i] = mask;
    }

    attack_masks
}

// generate one for white (idx 0) and one for black (idx 1)
pub fn generate_attack_masks_pawn() -> [[u64; 64]; 2] {
    let mut attack_masks = [[0; 64]; 2];

    for i in 0..64 {
        let mut mask = 0;
        let current = 1 << i;

        mask |= current.shift(Direction::UpLeft);
        mask |= current.shift(Direction::UpRight);

        // white
        attack_masks[0][i] = mask;
        
        // black
        attack_masks[1][i] = flip_bitboard_over_horizontal(mask);
    }

    attack_masks
}

pub fn generate_attack_masks_king() -> [u64; 64] {
    let mut attack_masks = [0; 64];

    for i in 0..64 {
        let mut mask = 0;
        let current = 1 << i;

        mask |= current.shift(Direction::Up);
        mask |= current.shift(Direction::UpRight);
        mask |= current.shift(Direction::Right);
        mask |= current.shift(Direction::DownRight);
        mask |= current.shift(Direction::Down);
        mask |= current.shift(Direction::DownLeft);
        mask |= current.shift(Direction::Left);
        mask |= current.shift(Direction::UpLeft);

        attack_masks[i] = mask;
    }

    attack_masks
}

pub fn generate_attack_masks_knight() -> [u64; 64] {
    let mut attack_masks = [0; 64];

    for i in 0..64 {
        let mut mask = 0;
        let current = 1 << i;

        mask |= current.shift(Direction::Up).shift(Direction::UpRight);
        mask |= current.shift(Direction::Up).shift(Direction::UpLeft);
        mask |= current.shift(Direction::Right).shift(Direction::UpRight);
        mask |= current.shift(Direction::Right).shift(Direction::DownRight);
        mask |= current.shift(Direction::Down).shift(Direction::DownRight);
        mask |= current.shift(Direction::Down).shift(Direction::DownLeft);
        mask |= current.shift(Direction::Left).shift(Direction::UpLeft);
        mask |= current.shift(Direction::Left).shift(Direction::DownLeft);

        attack_masks[i] = mask;
    }

    attack_masks
}

pub fn rotate_bitboard_90_clockwise(bitboard: u64) -> u64 {
    flip_bitboard_over_horizontal(flip_bitboard_diagonal_a8_h1(bitboard))
}

// what the fuck?
pub fn flip_bitboard_diagonal_a8_h1(mut bitboard: u64) -> u64 {
    let k1 = 0x5500550055005500;
    let k2 = 0x3333000033330000;
    let k4 = 0x0f0f0f0f00000000;
    let mut t = k4 & (bitboard ^ (bitboard << 28));
    bitboard ^= t ^ (t >> 28);
    t = k2 & (bitboard ^ (bitboard << 14));
    bitboard ^= t ^ (t >> 14);
    t = k1 & (bitboard ^ (bitboard << 7));
    bitboard ^= t ^ (t >> 7);

    bitboard
}

// what the fuck?
fn flip_bitboard_over_vertical(mut bitboard: u64) -> u64 {
    let k1 = 0x5555555555555555;
    let k2 = 0x3333333333333333;
    let k4 = 0x0f0f0f0f0f0f0f0f;
    bitboard = ((bitboard >> 1) & k1) +  2*(bitboard & k1);
    bitboard = ((bitboard >> 2) & k2) +  4*(bitboard & k2);
    bitboard = ((bitboard >> 4) & k4) + 16*(bitboard & k4);
    
    bitboard
 }

pub fn flip_bitboard_over_horizontal(bitboard: u64) -> u64 {
    bitboard.swap_bytes()
}

pub fn print_bitboard(bitboard: u64) {
    let bitboard = flip_bitboard_over_vertical(bitboard);
    for i in 0..8 {
        println!("{:08b}", (bitboard << i * 8) >> 7 * 8)
    }
    println!()
}