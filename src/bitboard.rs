pub fn generate_attack_masks_rook() -> [u64; 64] {
    let mut attack_masks = [0; 64];

    for i in 0..64 {
        let mut mask = 0;
        mask |= 0xff << (i / 8) * 8;
        
        let vertical_ray = (0b10000000 >> (i / 8) ) ;
        print_bitboard(vertical_ray);
        println!();
        // mask |= ;

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
    bitboard.to_be()
}

pub fn print_bitboard(bitboard: u64) {
    let bitboard = flip_bitboard_over_vertical(bitboard);
    for i in 0..8 {
        println!("{:08b}", (bitboard << i * 8) >> 7 * 8)
    }
}
