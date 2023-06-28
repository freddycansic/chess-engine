use crate::{
    bitboard::Bitboard,
    player::{Piece, Player},
};
use macroquad::prelude::*;
use rustc_hash::FxHashMap;

// the board should take up 80% of the height of the window
pub const BOARD_RATIO: f32 = 0.8;
pub const PIECE_SQUARE_RATIO: f32 = 0.9;

pub fn load_piece_textures() -> FxHashMap<Piece, Texture2D> {
    // scale up svg
    let transform = quad_svg::Transform::from_scale(5.0, 5.0);
    rustc_hash::FxHashMap::from_iter([
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
    ])
}

pub fn render_board(
    possible_moves: u64,
    white_player: &Player,
    black_player: &Player,
    board_length: f32,
    square_length: f32,
    board_start_x: f32,
    board_start_y: f32,
    piece_textures: &FxHashMap<Piece, Texture2D>,
) {
    let possible_moves_coordinates = possible_moves.to_2d_coordinates();

    for rank in 0..8 {
        for file in 0..8 {
            let square_start_x = board_start_x + file as f32 * square_length;
            let square_start_y = board_start_y + rank as f32 * square_length;

            draw_rectangle(
                square_start_x,
                square_start_y,
                square_length,
                square_length,
                if (rank + file) % 2 == 0 {
                    // chess com beige = white
                    macroquad::color_u8!(0xed, 0xed, 0xd1, 0xff)
                } else {
                    // chess com green = black
                    macroquad::color_u8!(0x75, 0x95, 0x57, 0xff)
                },
            );

            if possible_moves_coordinates.contains(&(file, 7 - rank)) {
                let radius = square_length * 0.2;

                draw_circle(
                    square_start_x + square_length / 2.0,
                    square_start_y + square_length / 2.0,
                    radius,
                    macroquad::color_u8!(0x00, 0x00, 0x00, 0x80),
                )
            }
        }
    }

    for player in [white_player, black_player] {
        for (piece_type, file, rank) in player.as_pieces() {
            let piece_length = square_length * PIECE_SQUARE_RATIO;
            let square_piece_difference = square_length - piece_length;

            // magic fix to translate coodinates given from as_pieces to screen coords
            let rank = 8 - rank + 1;

            // use draw_texture_ex for scaling
            draw_texture_ex(
                piece_textures[&piece_type],
                screen_width() / 2.0 - board_length / 2.0
                    + square_piece_difference / 2.0
                    + file as f32 * square_length,
                board_length - screen_height() / 2.0 - board_length / 2.0
                    + square_piece_difference / 2.0
                    + rank as f32 * square_length,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: piece_length,
                        y: piece_length,
                    }),
                    ..Default::default()
                },
            )
        }
    }
}
