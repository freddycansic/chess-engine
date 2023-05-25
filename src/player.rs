use crate::bitboard::Bitboard;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Piece {
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

pub struct Player {
    pub color: Color,
    pub pawn_bitboard: u64,
    pub knight_bitboard: u64,
    pub bishop_bitboard: u64,
    pub rook_bitboard: u64,
    pub queen_bitboard: u64,
    pub king_bitboard: u64,
}

impl Player {
    pub fn new(color: Color) -> Self {
        let pawn_bitboard = 0b1111111100000000;
        let knight_bitboard = 0b01000010;
        let bishop_bitboard = 0b00100100;
        let rook_bitboard = 0b10000001;
        let king_bitboard = 0b00010000;
        let queen_bitboard = 0b00001000;

        match color {
            Color::White => Player {
                color,
                pawn_bitboard,
                knight_bitboard,
                bishop_bitboard,
                rook_bitboard,
                queen_bitboard,
                king_bitboard,
            },
            Color::Black => Player {
                color: color,
                pawn_bitboard: pawn_bitboard.flip_over_horizontal(),
                knight_bitboard: knight_bitboard.flip_over_horizontal(),
                bishop_bitboard: bishop_bitboard.flip_over_horizontal(),
                rook_bitboard: rook_bitboard.flip_over_horizontal(),
                queen_bitboard: queen_bitboard.flip_over_horizontal(),
                king_bitboard: king_bitboard.flip_over_horizontal(),
            },
        }
    }

    pub fn all_bitboards(&self) -> u64 {
        self.pawn_bitboard
            | self.knight_bitboard
            | self.bishop_bitboard
            | self.rook_bitboard
            | self.queen_bitboard
            | self.king_bitboard
    }

    pub fn as_pieces(&self) -> Vec<(Piece, usize, usize)> {
        let mut piece_coords = Vec::new();

        let piece_type_to_bitboard = match self.color {
            Color::White => [
                (Piece::WhitePawn, self.pawn_bitboard),
                (Piece::WhiteKnight, self.knight_bitboard),
                (Piece::WhiteBishop, self.bishop_bitboard),
                (Piece::WhiteRook, self.rook_bitboard),
                (Piece::WhiteQueen, self.queen_bitboard),
                (Piece::WhiteKing, self.king_bitboard),
            ],

            Color::Black => [
                (Piece::BlackPawn, self.pawn_bitboard),
                (Piece::BlackKnight, self.knight_bitboard),
                (Piece::BlackBishop, self.bishop_bitboard),
                (Piece::BlackRook, self.rook_bitboard),
                (Piece::BlackQueen, self.queen_bitboard),
                (Piece::BlackKing, self.king_bitboard),
            ],
        };

        for (piece_type, piece_bitboard) in piece_type_to_bitboard.into_iter() {
            for i in 0..64 {
                let current = 1 << i;
                if current & piece_bitboard > 0 {
                    piece_coords.push((piece_type, i % 8, i / 8))
                }
            }
        }

        piece_coords
    }
}
