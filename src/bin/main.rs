use chess_engine::{bitboard::*, player::*};

use macroquad::{
    prelude::{is_mouse_button_down, mouse_position, MouseButton, Rect, Vec2, WHITE},
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

    let all_pawn_moves = generate_move_masks_pawn();
    let all_pawn_moves_white = all_pawn_moves[0];
    let all_pawn_moves_black = all_pawn_moves[1];
    let all_pawn_attack_moves = generate_attack_masks_pawn();
    let all_pawn_attack_moves_white = all_pawn_attack_moves[0];
    let all_pawn_attack_moves_black = all_pawn_attack_moves[1];
    let all_knight_moves = generate_move_masks_knight();
    let all_rook_moves = generate_move_masks_rook();
    let all_bishop_moves = generate_move_masks_bishop();
    let all_queen_moves = generate_move_masks_queen();
    let all_king_moves = generate_move_masks_king();

    let me_play_as = Color::White;

    loop {
        let board_length = screen_height() * BOARD_RATIO;
        let square_length = board_length / 8.0;
        let board_start_x = screen_width() / 2.0 - board_length / 2.0;
        let board_start_y = screen_height() / 2.0 - board_length / 2.0;

        clear_background(WHITE);

        for rank in 0..8 {
            for file in 0..8 {
                // have to use this in this scope because i have my own color struct whose constructor macro color_u8 is trying to call
                use macroquad::color::Color;
                draw_rectangle(
                    board_start_x + file as f32 * square_length,
                    board_start_y + rank as f32 * square_length,
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
            }
        }

        for player in [&white_player, &black_player] {
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
                let selected_square_bitboard = 1 << 8 * rank + file;

                let selected_player_piece_type_bitboard = match selected_square_bitboard {
                    bitboard if bitboard & white_player.pawn_bitboard > 0 => {
                        Some((&white_player, Piece::WhitePawn, white_player.pawn_bitboard))
                    }
                    bitboard if bitboard & white_player.knight_bitboard > 0 => Some((
                        &white_player,
                        Piece::WhiteKnight,
                        white_player.knight_bitboard,
                    )),
                    bitboard if bitboard & white_player.bishop_bitboard > 0 => Some((
                        &white_player,
                        Piece::WhiteBishop,
                        white_player.bishop_bitboard,
                    )),
                    bitboard if bitboard & white_player.rook_bitboard > 0 => {
                        Some((&white_player, Piece::WhiteRook, white_player.rook_bitboard))
                    }
                    bitboard if bitboard & white_player.queen_bitboard > 0 => Some((
                        &white_player,
                        Piece::WhiteQueen,
                        white_player.queen_bitboard,
                    )),
                    bitboard if bitboard & white_player.king_bitboard > 0 => {
                        Some((&white_player, Piece::WhiteKing, white_player.king_bitboard))
                    }
                    bitboard if bitboard & black_player.pawn_bitboard > 0 => {
                        Some((&black_player, Piece::BlackPawn, black_player.pawn_bitboard))
                    }
                    bitboard if bitboard & black_player.knight_bitboard > 0 => Some((
                        &black_player,
                        Piece::BlackKnight,
                        black_player.knight_bitboard,
                    )),
                    bitboard if bitboard & black_player.bishop_bitboard > 0 => Some((
                        &black_player,
                        Piece::BlackBishop,
                        black_player.bishop_bitboard,
                    )),
                    bitboard if bitboard & black_player.rook_bitboard > 0 => {
                        Some((&black_player, Piece::BlackRook, black_player.rook_bitboard))
                    }
                    bitboard if bitboard & black_player.queen_bitboard > 0 => Some((
                        &black_player,
                        Piece::BlackQueen,
                        black_player.queen_bitboard,
                    )),
                    bitboard if bitboard & black_player.king_bitboard > 0 => {
                        Some((&black_player, Piece::BlackKing, black_player.king_bitboard))
                    }
                    _ => None,
                };

                if let Some((selected_player, selected_piece, selected_bitboard)) =
                    selected_player_piece_type_bitboard
                {
                    println!("{:?} {:?}", selected_player.color, selected_piece);

                    let board_index = rank * 8 + file;
                    let white_bitboard = white_player.all_bitboards();
                    let black_bitboard = black_player.all_bitboards();
                    let whole_bitboard = white_bitboard | black_bitboard;

                    let (friendly_board, enemy_board) = match me_play_as {
                        Color::White => (white_bitboard, black_bitboard),
                        Color::Black => (black_bitboard, white_bitboard),
                    };

                    let possible_moves = match selected_piece {
                        // TODO pawn takes moves
                        Piece::WhitePawn => possible_moves_pawn(&all_pawn_moves_white, &all_pawn_attack_moves_white, me_play_as),
                        Piece::BlackPawn => possible_moves_pawn(&all_pawn_moves_black, &all_pawn_attack_moves_black, me_play_as),
                        Piece::WhiteKnight | Piece::BlackKnight => {
                            all_knight_moves[board_index] & !friendly_board
                        }
                        Piece::WhiteKing | Piece::BlackKing => {
                            all_king_moves[board_index] & !friendly_board
                        }
                        Piece::WhiteBishop | Piece::BlackBishop => possible_moves_bishop(
                            enemy_board,
                            whole_bitboard,
                            &all_bishop_moves,
                            board_index,
                        ),
                        Piece::WhiteQueen | Piece::BlackQueen => possible_moves_queen(
                            enemy_board,
                            whole_bitboard,
                            &all_queen_moves,
                            board_index,
                        ),
                        Piece::WhiteRook | Piece::BlackRook => possible_moves_rook(
                            enemy_board,
                            whole_bitboard,
                            &all_rook_moves,
                            board_index,
                        ),
                    };

                    // for rook bishop queen bitscan from current position in all directions until a 1 is hit, all 0s before then become 1s and valid moves

                    print_bitboard(possible_moves);
                }
            }
        }

        next_frame().await
    }
}
