use chess_engine::{bitboard::*, player::*, rendering::*};

use macroquad::{
    prelude::{
        is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton, Rect, Vec2,
        WHITE,
    },
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

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
    let piece_textures = load_piece_textures();

    let mut white_player = Player::new(Color::White);
    let mut black_player = Player::new(Color::Black);
    println!("{:#?}", white_player.bishop_bitboard.to_2d_coordinates());

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

    for pawn_move in all_pawn_moves_white {
        print_bitboard(pawn_move)
    }

    println!("BLACK");
    for pawn_move in all_pawn_moves_black {
        print_bitboard(pawn_move)
    }

    let mut me_play_as = Color::White;
    let mut current_start_bitboard = 0;
    let mut current_selected_piece = Piece::WhitePawn;
    let mut dragging_piece = false;
    let mut possible_moves = 0;

    loop {
        let board_length = screen_height() * BOARD_RATIO;
        let square_length = board_length / 8.0;
        let board_start_x = screen_width() / 2.0 - board_length / 2.0;
        let board_start_y = screen_height() / 2.0 - board_length / 2.0;

        clear_background(WHITE);

        render_board(
            possible_moves,
            &white_player,
            &black_player,
            board_length,
            square_length,
            board_start_x,
            board_start_y,
            &piece_textures,
        );

        let mouse_pos = mouse_position();
        let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);

        if Rect::new(board_start_x, board_start_y, board_length, board_length).contains(mouse_pos) {
            let file = ((mouse_pos.x - board_start_x) / square_length) as usize;
            // flip rank since mouse pos measured from top left and board measured from bottom left
            let rank = 7 - ((mouse_pos.y - board_start_y) / square_length) as usize;

            // get piece in square
            let hovered_square_bitboard = 1 << 8 * rank + file;

            if is_mouse_button_down(MouseButton::Left) && !dragging_piece {
                if let Some((selected_color, selected_piece, _selected_bitboard)) =
                    get_square_info(hovered_square_bitboard, &white_player, &black_player)
                {
                    current_start_bitboard = hovered_square_bitboard;
                    current_selected_piece = selected_piece;

                    if selected_color == me_play_as {
                        println!("{:?} {:?}", selected_color, selected_piece);

                        let board_index = rank * 8 + file;
                        let white_bitboard = white_player.all_bitboards();
                        let black_bitboard = black_player.all_bitboards();
                        let whole_bitboard = white_bitboard | black_bitboard;

                        let (friendly_bitboard, enemy_bitboard) = match me_play_as {
                            Color::White => (white_bitboard, black_bitboard),
                            Color::Black => (black_bitboard, white_bitboard),
                        };

                        possible_moves = get_possible_moves(
                            selected_piece,
                            whole_bitboard,
                            friendly_bitboard,
                            enemy_bitboard,
                            board_index,
                            &all_pawn_moves_white,
                            &all_pawn_attack_moves_white,
                            &all_pawn_moves_black,
                            &all_pawn_attack_moves_black,
                            &all_knight_moves,
                            &all_bishop_moves,
                            &all_rook_moves,
                            &all_queen_moves,
                            &all_king_moves,
                        );

                        println!("START POS");
                        print_bitboard(hovered_square_bitboard);

                        println!("POSSIBLE MOVES");
                        print_bitboard(possible_moves);

                        dragging_piece = true;
                    }
                }
            } else if dragging_piece && is_mouse_button_released(MouseButton::Left) {
                println!("DEST POS");
                print_bitboard(hovered_square_bitboard);

                if hovered_square_bitboard & possible_moves > 0 {
                    println!("VALID");
                    
                    if let Some((_destination_color, destination_piece, _destination_bitboard)) = get_square_info(hovered_square_bitboard, &white_player, &black_player) {
                        match destination_piece {
                            Piece::WhitePawn => white_player.pawn_bitboard &= !hovered_square_bitboard,
                            Piece::WhiteKnight => white_player.knight_bitboard &= !hovered_square_bitboard,
                            Piece::WhiteBishop => white_player.bishop_bitboard &= !hovered_square_bitboard,
                            Piece::WhiteRook => white_player.rook_bitboard &= !hovered_square_bitboard,
                            Piece::WhiteQueen => white_player.queen_bitboard &= !hovered_square_bitboard,
                            Piece::WhiteKing => white_player.king_bitboard &= !hovered_square_bitboard,
                            Piece::BlackPawn => black_player.pawn_bitboard &= !hovered_square_bitboard,
                            Piece::BlackKnight => black_player.knight_bitboard &= !hovered_square_bitboard,
                            Piece::BlackBishop => black_player.bishop_bitboard &= !hovered_square_bitboard,
                            Piece::BlackRook => black_player.rook_bitboard &= !hovered_square_bitboard,
                            Piece::BlackQueen => black_player.queen_bitboard &= !hovered_square_bitboard,
                            Piece::BlackKing => black_player.king_bitboard &= !hovered_square_bitboard
                        };
                    }

                    let move_piece = |bitboard: &mut u64| {
                        *bitboard &= !current_start_bitboard;
                        *bitboard |= hovered_square_bitboard
                    };

                    match current_selected_piece {
                        Piece::WhitePawn => move_piece(&mut white_player.pawn_bitboard),
                        Piece::WhiteKnight => move_piece(&mut white_player.knight_bitboard),
                        Piece::WhiteBishop => move_piece(&mut white_player.bishop_bitboard),
                        Piece::WhiteRook => move_piece(&mut white_player.rook_bitboard),
                        Piece::WhiteQueen => move_piece(&mut white_player.queen_bitboard),
                        Piece::WhiteKing => move_piece(&mut white_player.king_bitboard),
                        Piece::BlackPawn => move_piece(&mut black_player.pawn_bitboard),
                        Piece::BlackKnight => move_piece(&mut black_player.knight_bitboard),
                        Piece::BlackBishop => move_piece(&mut black_player.bishop_bitboard),
                        Piece::BlackRook => move_piece(&mut black_player.rook_bitboard),
                        Piece::BlackQueen => move_piece(&mut black_player.queen_bitboard),
                        Piece::BlackKing => move_piece(&mut black_player.king_bitboard)
                    };

                    if me_play_as == Color::White {
                        me_play_as = Color::Black
                    } else {
                        me_play_as = Color::White
                    }
                }

                dragging_piece = false;
                possible_moves = 0
            }
        }

        next_frame().await
    }
}

fn get_possible_moves(
    selected_piece: Piece,
    whole_bitboard: u64,
    friendly_bitboard: u64,
    enemy_bitboard: u64,
    board_index: usize,
    all_pawn_moves_white: &[u64; 64],
    all_pawn_attack_moves_white: &[u64; 64],
    all_pawn_moves_black: &[u64; 64],
    all_pawn_attack_moves_black: &[u64; 64],
    all_knight_moves: &[u64; 64],
    all_bishop_moves: &[u64; 64],
    all_rook_moves: &[u64; 64],
    all_queen_moves: &[u64; 64],
    all_king_moves: &[u64; 64],
) -> u64 {
    match selected_piece {
        // TODO pawn takes moves
        Piece::WhitePawn => possible_moves_pawn(
            enemy_bitboard,
            whole_bitboard,
            &all_pawn_moves_white,
            &all_pawn_attack_moves_white,
            board_index,
        ),
        Piece::BlackPawn => possible_moves_pawn(
            enemy_bitboard,
            whole_bitboard,
            &all_pawn_moves_black,
            &all_pawn_attack_moves_black,
            board_index,
        ),
        Piece::WhiteKnight | Piece::BlackKnight => {
            all_knight_moves[board_index] & !friendly_bitboard
        }
        Piece::WhiteKing | Piece::BlackKing => all_king_moves[board_index] & !friendly_bitboard,
        Piece::WhiteBishop | Piece::BlackBishop => possible_moves_bishop(
            enemy_bitboard,
            whole_bitboard,
            &all_bishop_moves,
            board_index,
        ),
        Piece::WhiteQueen | Piece::BlackQueen => possible_moves_queen(
            enemy_bitboard,
            whole_bitboard,
            &all_queen_moves,
            board_index,
        ),
        Piece::WhiteRook | Piece::BlackRook => {
            possible_moves_rook(enemy_bitboard, whole_bitboard, &all_rook_moves, board_index)
        }
    }
}

fn get_square_info<'a>(
    selected_square_bitboard: u64,
    white_player: &'a Player,
    black_player: &'a Player,
) -> Option<(Color, Piece, u64)> {
    match selected_square_bitboard {
        bitboard if bitboard & white_player.pawn_bitboard > 0 => {
            Some((Color::White, Piece::WhitePawn, white_player.pawn_bitboard))
        }
        bitboard if bitboard & white_player.knight_bitboard > 0 => Some((
            Color::White,
            Piece::WhiteKnight,
            white_player.knight_bitboard,
        )),
        bitboard if bitboard & white_player.bishop_bitboard > 0 => Some((
            Color::White,
            Piece::WhiteBishop,
            white_player.bishop_bitboard,
        )),
        bitboard if bitboard & white_player.rook_bitboard > 0 => {
            Some((Color::White, Piece::WhiteRook, white_player.rook_bitboard))
        }
        bitboard if bitboard & white_player.queen_bitboard > 0 => {
            Some((Color::White, Piece::WhiteQueen, white_player.queen_bitboard))
        }
        bitboard if bitboard & white_player.king_bitboard > 0 => {
            Some((Color::White, Piece::WhiteKing, white_player.king_bitboard))
        }
        bitboard if bitboard & black_player.pawn_bitboard > 0 => {
            Some((Color::Black, Piece::BlackPawn, black_player.pawn_bitboard))
        }
        bitboard if bitboard & black_player.knight_bitboard > 0 => Some((
            Color::Black,
            Piece::BlackKnight,
            black_player.knight_bitboard,
        )),
        bitboard if bitboard & black_player.bishop_bitboard > 0 => Some((
            Color::Black,
            Piece::BlackBishop,
            black_player.bishop_bitboard,
        )),
        bitboard if bitboard & black_player.rook_bitboard > 0 => {
            Some((Color::Black, Piece::BlackRook, black_player.rook_bitboard))
        }
        bitboard if bitboard & black_player.queen_bitboard > 0 => {
            Some((Color::Black, Piece::BlackQueen, black_player.queen_bitboard))
        }
        bitboard if bitboard & black_player.king_bitboard > 0 => {
            Some((Color::Black, Piece::BlackKing, black_player.king_bitboard))
        }
        _ => None,
    }
}
