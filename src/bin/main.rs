use chess_engine::{bitboard::*, player::*};

use macroquad::{
    prelude::{is_mouse_button_down, mouse_position, MouseButton, Rect, Vec2, BLUE, RED, WHITE},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

// the board should take up 80% of the height of the window
const BOARD_RATIO: f32 = 0.8;
const PIECE_SQUARE_RATIO: f32 = 0.9;

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
    // FILE_A.as_2d_coords();

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

    loop {
        let board_length = screen_height() * BOARD_RATIO;
        let square_length = board_length / 8.0;
        let board_start_x = screen_width() / 2.0 - board_length / 2.0;
        let board_start_y = screen_height() / 2.0 - board_length / 2.0;

        clear_background(WHITE);

        for rank in 0..8 {
            for file in 0..8 {
                draw_rectangle(
                    board_start_x + file as f32 * square_length,
                    board_start_y + rank as f32 * square_length,
                    square_length,
                    square_length,
                    if (rank + file) % 2 == 0 { RED } else { BLUE },
                );
            }
        }

        for player in [&white_player, &black_player] {
            for (piece_type, file, rank) in player.as_pieces() {
                let piece_length = square_length * PIECE_SQUARE_RATIO;
                let square_piece_difference = square_length - piece_length;

                // magic fix to translate coodinates given from as_pieces to screen coords
                let file = 8 - file - 1;
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

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);

            if Rect::new(board_start_x, board_start_y, board_length, board_length)
                .contains(mouse_pos)
            {
                let file = ((mouse_pos.x - board_start_x) / square_length) as usize;
                // flip rank since mouse pos measured from top left and board measured from bottom left
                let rank = 7 - ((mouse_pos.y - board_start_y) / square_length) as usize;

                // get piece in square
            }
        }

        next_frame().await
    }
}
