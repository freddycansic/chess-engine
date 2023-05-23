use chess_engine::bitboard;

use macroquad::{
    prelude::{is_mouse_button_down, mouse_position, MouseButton, Rect, Vec2, BLUE, RED, WHITE},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

use bitboard::{flip_bitboard_over_horizontal, generate_attack_masks_rook, print_bitboard};

// the board should take up 80% of the height of the window
const BOARD_RATIO: f32 = 0.8;
const PIECE_SQUARE_RATIO: f32 = 0.9;

#[derive(Hash, PartialEq, Eq)]
enum Piece {
    None,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}
struct Player {
    color: Color,
    pawn_bitboard: u64,
    knight_bitboard: u64,
    bishop_bitboard: u64,
    rook_bitboard: u64,
    queen_bitboard: u64,
    king_bitboard: u64,
}

impl Player {
    fn new(color: Color) -> Self {
        let pawn_bitboard = 0b1111111100000000;
        let knight_bitboard = 0b01000010;
        let bishop_bitboard = 0b00100100;
        let rook_bitboard = 0b10000001;
        let king_bitboard = 0b00001000;
        let queen_bitboard = 0b00010000;

        match color {
            Color::White => Player {
                color: color,
                pawn_bitboard: pawn_bitboard,
                knight_bitboard: knight_bitboard,
                bishop_bitboard: bishop_bitboard,
                rook_bitboard: rook_bitboard,
                queen_bitboard: queen_bitboard,
                king_bitboard: king_bitboard,
            },
            Color::Black => Player {
                color: color,
                pawn_bitboard: flip_bitboard_over_horizontal(pawn_bitboard),
                knight_bitboard: flip_bitboard_over_horizontal(knight_bitboard),
                bishop_bitboard: flip_bitboard_over_horizontal(bishop_bitboard),
                rook_bitboard: flip_bitboard_over_horizontal(rook_bitboard),
                queen_bitboard: flip_bitboard_over_horizontal(queen_bitboard),
                king_bitboard: flip_bitboard_over_horizontal(king_bitboard),
            },
        }
    }

    fn all_bitboards(&self) -> u64 {
        self.pawn_bitboard
            | self.knight_bitboard
            | self.bishop_bitboard
            | self.rook_bitboard
            | self.queen_bitboard
            | self.king_bitboard
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Engine".to_owned(),
        fullscreen: false,
        window_height: 1000,
        window_width: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    for mask in bitboard::generate_attack_masks_knight().into_iter() {
        print_bitboard(mask)
    }

    // scale up svg
    let transform = quad_svg::Transform::from_scale(5.0, 5.0);
    let piece_textures = rustc_hash::FxHashMap::from_iter([
        (
            Piece::WhitePawn,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/white_pawn.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::WhiteKnight,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/white_knight.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::WhiteBishop,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/white_bishop.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::WhiteRook,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/white_rook.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::WhiteQueen,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/white_queen.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::WhiteKing,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/white_king.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::BlackPawn,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/black_pawn.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::BlackKnight,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/black_knight.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::BlackBishop,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/black_bishop.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::BlackRook,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/black_rook.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::BlackQueen,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/black_queen.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
        (
            Piece::BlackKing,
            quad_svg::svg_to_texture(
                &std::fs::read_to_string("res/black_king.svg").unwrap(),
                &transform,
            )
            .unwrap(),
        ),
    ]);

    let white_player = Player::new(Color::White);
    let black_player = Player::new(Color::Black);

    // print_bitboard(white_player.bishop_bitboard);
    // println!();
    // print_bitboard(white_player.pawn_bitboard);
    // print_bitboard(white_player.all_bitboards())

    // print_bitboard(white_player.knight_bitboard);
    // println!();
    // print_bitboard(black_player.knight_bitboard);
    // print_bitboard(white_player.knight_bitboard ^ 56);
    // print_bitboard(black_player.knight_bitboard);

    // loop {
    //     let board_length = screen_height() * BOARD_RATIO;
    //     let square_length = board_length / 8.0;
    //     let board_start_x = screen_width() / 2.0 - board_length / 2.0;
    //     let board_start_y = screen_height() / 2.0 - board_length / 2.0;

    //     clear_background(WHITE);

    //     for rank in 0..8 {
    //         for file in 0..8 {
    //             draw_rectangle(
    //                 board_start_x + file as f32 * square_length,
    //                 board_start_y + rank as f32 * square_length,
    //                 square_length,
    //                 square_length,
    //                 if (rank + file) % 2 == 0 { RED } else { BLUE },
    //             );
    //         }
    //     }

    //     for rank in 0..8 {
    //         for file in 0..8 {
    //             let current_piece = &board[file][rank];
    //             if *current_piece == Piece::None {
    //                 continue;
    //             }

    //             let piece_length = square_length * PIECE_SQUARE_RATIO;
    //             let square_piece_difference = square_length - piece_length;

    //             // use draw_texture_ex for scaling
    //             draw_texture_ex(
    //                 piece_textures[current_piece],
    //                 screen_width() / 2.0 - board_length / 2.0
    //                     + square_piece_difference / 2.0
    //                     + file as f32 * square_length,
    //                 screen_height() / 2.0 - board_length / 2.0
    //                     + square_piece_difference / 2.0
    //                     + rank as f32 * square_length,
    //                 WHITE,
    //                 DrawTextureParams {
    //                     dest_size: Some(Vec2 {
    //                         x: piece_length,
    //                         y: piece_length,
    //                     }),
    //                     ..Default::default()
    //                 },
    //             )
    //         }
    //     }

    //     if is_mouse_button_down(MouseButton::Left) {
    //         let mouse_pos = mouse_position();
    //         let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);

    //         if Rect::new(board_start_x, board_start_y, board_length, board_length)
    //             .contains(mouse_pos)
    //         {
    //             let file = ((mouse_pos.x - board_start_x) / square_length) as usize;
    //             // flip rank since mouse pos measured from top left and board measured from bottom left
    //             let rank = 7 - ((mouse_pos.y - board_start_y) / square_length) as usize;

    //             // get piece in square
    //         }
    //     }

    //     next_frame().await
    // }
}
