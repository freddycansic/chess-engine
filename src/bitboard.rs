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

pub trait Bitboard {
    fn shift(&self, direction: Direction) -> Self;
    fn flip_over_horizontal(&self) -> Self;
    fn flip_over_vertical(&self) -> Self;
    fn flip_diagonal_a8_h1(&self) -> Self;
    fn rotate_90_clockwise(&self) -> Self;
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

    fn rotate_90_clockwise(&self) -> u64 {
        self.flip_diagonal_a8_h1().flip_over_horizontal()
    }

    // what the fuck?
    fn flip_diagonal_a8_h1(&self) -> u64 {
        let mut bitboard = *self;

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
    fn flip_over_vertical(&self) -> u64 {
        let mut bitboard = *self;

        let k1 = 0x5555555555555555;
        let k2 = 0x3333333333333333;
        let k4 = 0x0f0f0f0f0f0f0f0f;
        bitboard = ((bitboard >> 1) & k1) + 2 * (bitboard & k1);
        bitboard = ((bitboard >> 2) & k2) + 4 * (bitboard & k2);
        bitboard = ((bitboard >> 4) & k4) + 16 * (bitboard & k4);

        bitboard
    }

    fn flip_over_horizontal(&self) -> u64 {
        self.swap_bytes()
    }
}

pub fn print_bitboard(bitboard: u64) {
    let bitboard = bitboard.flip_over_vertical();
    for i in 0..8 {
        println!("{:08b}", (bitboard << i * 8) >> 7 * 8)
    }
    println!()
}

pub fn generate_move_masks_rook() -> [u64; 64] {
    let mut move_masks = [0; 64];

    for i in 0..64 {
        let mut mask = 0;
        mask |= RANK_1 << (i / 8) * 8;
        mask |= FILE_A << i % 8;
        mask &= !(1 << i);

        move_masks[i] = mask;
    }

    move_masks
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
        attack_masks[1][i] = mask.flip_over_horizontal();
    }

    attack_masks
}

pub fn generate_move_masks_pawn() -> [[u64; 64]; 2] {
    let mut move_masks = [[0; 64]; 2];

    for i in 0..64 {
        let mut mask = 0;
        let current = 1 << i;

        mask |= current.shift(Direction::Up);
        mask |= (current & RANK_2).shift(Direction::Up).shift(Direction::Up);

        move_masks[0][i] = mask;
        move_masks[1][i] = mask.flip_over_horizontal();
    }

    move_masks[1].reverse();

    move_masks
}

pub fn generate_move_masks_king() -> [u64; 64] {
    let mut move_masks = [0; 64];

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

        move_masks[i] = mask;
    }

    move_masks
}

pub fn generate_move_masks_knight() -> [u64; 64] {
    let mut move_masks = [0; 64];

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

        move_masks[i] = mask;
    }

    move_masks
}

pub fn generate_move_masks_bishop() -> [u64; 64] {
    let mut move_masks = [0; 64];

    for i in 0..64 {
        let mut mask = 0;
        let current = 1_u64 << i;

        let mut up_left_ray = current.shift(Direction::UpLeft);
        let mut up_right_ray = current.shift(Direction::UpRight);
        let mut down_left_ray = current.shift(Direction::DownLeft);
        let mut down_right_ray = current.shift(Direction::DownRight);
        // bruteforce rays past the edge of the board cause who cares anymore
        for _ in 0..7 {
            up_left_ray |= up_left_ray.shift(Direction::UpLeft);
            up_right_ray |= up_right_ray.shift(Direction::UpRight);
            down_left_ray |= down_left_ray.shift(Direction::DownLeft);
            down_right_ray |= down_right_ray.shift(Direction::DownRight);
        }

        mask |= up_left_ray | up_right_ray | down_left_ray | down_right_ray;

        move_masks[i] = mask;
    }

    move_masks
}

pub fn generate_move_masks_queen() -> [u64; 64] {
    let mut move_masks = [0; 64];

    // do it again cause who cares anymore
    let rook_masks = generate_move_masks_rook();
    let bishop_masks = generate_move_masks_bishop();

    for i in 0..64 {
        move_masks[i] = rook_masks[i] | bishop_masks[i];
    }

    move_masks
}

pub fn possible_moves_bishop(
    enemy_bitboard: u64,
    whole_bitboard: u64,
    all_bishop_moves: &[u64; 64],
    board_index: usize,
) -> u64 {
    let possible_moves = all_bishop_moves[board_index];
    let blockers_bitboard = possible_moves & whole_bitboard;
    let current = 1 << board_index;

    let up_left_ray = closest_sliding_moves(Direction::UpLeft, Direction::DownRight, enemy_bitboard, blockers_bitboard, current);
    let up_right_ray = closest_sliding_moves(Direction::UpRight, Direction::DownLeft, enemy_bitboard, blockers_bitboard, current);
    let down_left_ray = closest_sliding_moves(Direction::DownLeft, Direction::UpRight, enemy_bitboard, blockers_bitboard, current);
    let down_right_ray = closest_sliding_moves(Direction::DownRight, Direction::UpLeft, enemy_bitboard, blockers_bitboard, current);

    up_left_ray | up_right_ray | down_left_ray | down_right_ray
}

pub fn possible_moves_rook(
    enemy_bitboard: u64,
    whole_bitboard: u64,
    all_rook_moves: &[u64; 64],
    board_index: usize,
) -> u64 {
    let possible_moves = all_rook_moves[board_index];
    let blockers_bitboard = possible_moves & whole_bitboard;
    let current = 1 << board_index;

    let left_ray = closest_sliding_moves(Direction::Left, Direction::Right, enemy_bitboard, blockers_bitboard, current);
    let right_ray = closest_sliding_moves(Direction::Right, Direction::Left, enemy_bitboard, blockers_bitboard, current);
    let up_ray = closest_sliding_moves(Direction::Up, Direction::Down, enemy_bitboard, blockers_bitboard, current);
    let down_ray = closest_sliding_moves(Direction::Down, Direction::Up, enemy_bitboard, blockers_bitboard, current);

    left_ray | right_ray | up_ray | down_ray
}

pub fn possible_moves_queen(
    enemy_bitboard: u64,
    whole_bitboard: u64,
    all_queen_moves: &[u64; 64],
    board_index: usize,
) -> u64 {
    let possible_moves = all_queen_moves[board_index];
    let blockers_bitboard = possible_moves & whole_bitboard;
    let current = 1 << board_index;

    let left_ray = closest_sliding_moves(Direction::Left, Direction::Right, enemy_bitboard, blockers_bitboard, current);
    let right_ray = closest_sliding_moves(Direction::Right, Direction::Left, enemy_bitboard, blockers_bitboard, current);
    let up_ray = closest_sliding_moves(Direction::Up, Direction::Down, enemy_bitboard, blockers_bitboard, current);
    let down_ray = closest_sliding_moves(Direction::Down, Direction::Up, enemy_bitboard, blockers_bitboard, current);
    let up_left_ray = closest_sliding_moves(Direction::UpLeft, Direction::DownRight, enemy_bitboard, blockers_bitboard, current);
    let up_right_ray = closest_sliding_moves(Direction::UpRight, Direction::DownLeft, enemy_bitboard, blockers_bitboard, current);
    let down_left_ray = closest_sliding_moves(Direction::DownLeft, Direction::UpRight, enemy_bitboard, blockers_bitboard, current);
    let down_right_ray = closest_sliding_moves(Direction::DownRight, Direction::UpLeft, enemy_bitboard, blockers_bitboard, current);

    up_left_ray | up_right_ray | down_left_ray | down_right_ray | left_ray | right_ray | up_ray | down_ray
}

fn closest_sliding_moves(ray_direction: Direction, opposite_direction: Direction, enemy_bitboard: u64, blockers_bitboard: u64, current: u64) -> u64 {
    // cast a ray, if we hit an enemy blocker then return, if we hit a friendly blocker then shift the ray back one place as to not allow taking of own pieces and return
    let mut current_ray = current.shift(ray_direction);
    
    for _ in 0..8 {
        if enemy_bitboard & current_ray & blockers_bitboard > 0 {
            break;
        } else if current_ray & blockers_bitboard > 0 {
            current_ray = current_ray.shift(opposite_direction);
            break;
        }
        current_ray |= current_ray.shift(ray_direction);
    }

    current_ray & !current
}