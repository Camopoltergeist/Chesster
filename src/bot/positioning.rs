//! Piece-square tables for all PlayerPiece variants.

use crate::{board::{position::Position, tile_position::TilePosition}, piece::PieceType, player::Player, player_piece::PlayerPiece};

pub const BLACK_KING_PIECE_SQUARE_TABLE: [(i32,i32); 64] = [
    (-80,-20),(-70,-10),(-70,-10),(-70,-10),(-70,-10),(-70,-10),(-70,-10),(-80,-20),
    (-60, -5),(-60,  0),(-60,  5),(-60,  5),(-60,  5),(-60,  5),(-60,  0),(-60, -5),
    (-40,-10),(-50, -5),(-50, 20),(-60, 30),(-60, 30),(-50, 20),(-50, -5),(-40,-10),
    (-30,-15),(-40,-10),(-40, 35),(-50, 45),(-50, 45),(-40, 35),(-40,-10),(-30,-15),
    (-20,-20),(-30,-15),(-30, 30),(-40, 40),(-40, 40),(-30, 30),(-30,-15),(-20,-20),
    (-10,-25),(-20,-20),(-20, 20),(-20, 25),(-20, 25),(-20, 20),(-20,-20),(-10,-25),
    (20, -35),( 20,-25),( -5,  0),( -5,  0),( -5,  0),( -5,  0),(20, -25),(20, -30),
    (20, -50),(30, -30),(10 ,-30),( 0, -30),(  0,-30),( 10,-30),(30, -30),(20, -50)
];

pub const WHITE_KING_PIECE_SQUARE_TABLE: [(i32,i32); 64] = [
    (20, -50),(30, -30),(10 ,-30),( 0, -30),(  0,-30),( 10,-30),(30, -30),(20, -50),
    (20, -35),( 20,-25),( -5,  0),( -5,  0),( -5,  0),( -5,  0),(20, -25),(20, -30),
    (-10,-25),(-20,-20),(-20, 20),(-20, 25),(-20, 25),(-20, 20),(-20,-20),(-10,-25),
    (-20,-20),(-30,-15),(-30, 30),(-40, 40),(-40, 40),(-30, 30),(-30,-15),(-20,-20),
    (-30,-15),(-40,-10),(-40, 35),(-50, 45),(-50, 45),(-40, 35),(-40,-10),(-30,-15),
    (-40,-10),(-50, -5),(-50, 20),(-60, 30),(-60, 30),(-50, 20),(-50, -5),(-40,-10),
    (-60, -5),(-60,  0),(-60,  5),(-60,  5),(-60,  5),(-60,  5),(-60,  0),(-60, -5),
    (-80,-20),(-70,-10),(-70,-10),(-70,-10),(-70,-10),(-70,-10),(-70,-10),(-80,-20)
];

pub const KING_PIECE_SQUARE_TABLE_CHECKMATE: [i32; 64] = [
    -100,-80, -60, -30, -30, -60, -80,-100,
    -80, -70, -40, -20, -20, -40, -70, -80,
    -60, -40, -20, -10, -10, -20, -40, -60,
    -50, -30, -10,   0,   0, -10, -30, -50,
    -50, -30, -10,   0,   0, -10, -30, -50,
    -60, -40, -20, -10, -10, -20, -40, -60,
    -80, -70, -60, -20, -20, -60, -70, -80,
    -100,-80, -60, -30, -30, -60, -80,-100,
];


pub const BLACK_PAWN_PIECE_SQUARE_TABLE: [(i32,i32); 64] = [
    (0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),
    (50,80),(50,80),(50,80),(50,80),(50,80),(50,80),(50,80),(50,80),
    (10,50),(10,50),(20,50),(30,50),(30,50),(20,50),(10,50),(10,50),
    ( 5,30),( 5,30),(10,30),(25,30),(25,30),(10,30),( 5,30),( 5,30),
    ( 0,20),( 0,20),( 0,20),(20,20),(20,20),( 0,20),( 0,20),( 0,20),
    ( 5,10),(-5,10),(-10,10),(0,10),( 0,10),(-10,10),(-5,10),(5,10),
    ( 5,10),(10,10),(10,10),(-20,10),(-20,10),(10,10),(10,10),(5,10),
    (0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)
];

pub const WHITE_PAWN_PIECE_SQUARE_TABLE: [(i32,i32); 64] = [
    (0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),
    ( 5,10),(10,10),(10,10),(-20,10),(-20,10),(10,10),(10,10),(5,10),
    ( 5,10),(-5,10),(-10,10),(0,10),( 0,10),(-10,10),(-5,10),(5,10),
    ( 0,20),( 0,20),( 0,20),(20,20),(20,20),( 0,20),( 0,20),( 0,20),
    ( 5,30),( 5,30),(10,30),(25,30),(25,30),(10,30),( 5,30),( 5,30),
    (10,50),(10,50),(20,50),(30,50),(30,50),(20,50),(10,50),(10,50),
    (50,80),(50,80),(50,80),(50,80),(50,80),(50,80),(50,80),(50,80),
    (0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)
];

pub const BISHOP_PIECE_SQUARE_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub const BLACK_ROOK_PIECE_SQUARE_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

pub const WHITE_ROOK_PIECE_SQUARE_TABLE: [i32; 64] = [
    0,  0,  0,  5,  5,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    5, 10, 10, 10, 10, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0,
];

pub const QUEEN_PIECE_SQUARE_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    -5,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

pub const KNIGHT_PIECE_SQUARE_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub fn get_score_for_piece(piece: PlayerPiece, tile_position: TilePosition, game_phase: (i32, i32), position: &Position) -> i32 {
    let index = tile_position.bit_offset() as usize;

    match piece.player() {
        Player::White => {
            match piece.piece() {
                PieceType::King => calculate_score_for_white_king(index, game_phase, position),
                PieceType::Pawn => calculate_score_for_white_pawn(index, game_phase),
                PieceType::Rook => WHITE_ROOK_PIECE_SQUARE_TABLE[index],
                PieceType::Bishop => BISHOP_PIECE_SQUARE_TABLE[index],
                PieceType::Knight => KNIGHT_PIECE_SQUARE_TABLE[index],
                PieceType::Queen => QUEEN_PIECE_SQUARE_TABLE[index]
            }
        },
        Player::Black => {
            match piece.piece() {
                PieceType::King => calculate_score_for_black_king(index, game_phase, position),
                PieceType::Pawn => calculate_score_for_black_pawn(index, game_phase),
                PieceType::Rook => BLACK_ROOK_PIECE_SQUARE_TABLE[index],
                PieceType::Bishop => BISHOP_PIECE_SQUARE_TABLE[index],
                PieceType::Knight => KNIGHT_PIECE_SQUARE_TABLE[index],
                PieceType::Queen => QUEEN_PIECE_SQUARE_TABLE[index]
            }
        }
    }
}

pub fn calculate_score_for_white_king(index: usize, game_phase: (i32, i32), position: &Position) -> i32 {
    let player_board = *position.board().get_player_bitboard(Player::White);
    let rooks_and_queens = position.board().queens | position.board().rooks;

    if player_board & rooks_and_queens == 0 {
        return KING_PIECE_SQUARE_TABLE_CHECKMATE[index];
    }
    
    let square_values = WHITE_KING_PIECE_SQUARE_TABLE[index];
    let total_value = game_phase.0 * square_values.0 + game_phase.1 * square_values.1;

    total_value / 100
}

pub fn calculate_score_for_black_king(index: usize, game_phase: (i32, i32), position: &Position) -> i32 {
    let player_board = *position.board().get_player_bitboard(Player::Black);
    let rooks_and_queens = position.board().queens | position.board().rooks;

    if player_board & rooks_and_queens == 0 {
        return KING_PIECE_SQUARE_TABLE_CHECKMATE[index];
    }
    
    let square_values = BLACK_KING_PIECE_SQUARE_TABLE[index];
    let total_value = game_phase.0 * square_values.0 + game_phase.1 * square_values.1;
    total_value / 100
}

pub fn calculate_score_for_white_pawn(index: usize, game_phase: (i32, i32)) -> i32 {
    let square_values = WHITE_PAWN_PIECE_SQUARE_TABLE[index];
    let total_value = game_phase.0 * square_values.0 + game_phase.1 * square_values.1;
    total_value / 100
}

pub fn calculate_score_for_black_pawn(index: usize, game_phase: (i32, i32)) -> i32 {
    let square_values = BLACK_PAWN_PIECE_SQUARE_TABLE[index];
    let total_value = game_phase.0 * square_values.0 + game_phase.1 * square_values.1;
    total_value / 100
}