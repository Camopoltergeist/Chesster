use crate::{board::position::Position, piece::PieceType, player::Player, player_piece::PlayerPiece};

#[test]
fn start_has_no_overlaps() {
    let position = Position::default();

    assert!(position.board().validate());
}

#[test]
fn start_has_correct_piece_placement() {
    let position = Position::default();

    // Rank 1
    assert_eq!(
        position.get_piece_debug("a1"),
        Some(PlayerPiece::new(Player::White, PieceType::Rook))
    );
    assert_eq!(
        position.get_piece_debug("b1"),
        Some(PlayerPiece::new(Player::White, PieceType::Knight))
    );
    assert_eq!(
        position.get_piece_debug("c1"),
        Some(PlayerPiece::new(Player::White, PieceType::Bishop))
    );
    assert_eq!(
        position.get_piece_debug("d1"),
        Some(PlayerPiece::new(Player::White, PieceType::Queen))
    );

    assert_eq!(
        position.get_piece_debug("e1"),
        Some(PlayerPiece::new(Player::White, PieceType::King))
    );
    assert_eq!(
        position.get_piece_debug("f1"),
        Some(PlayerPiece::new(Player::White, PieceType::Bishop))
    );
    assert_eq!(
        position.get_piece_debug("g1"),
        Some(PlayerPiece::new(Player::White, PieceType::Knight))
    );
    assert_eq!(
        position.get_piece_debug("h1"),
        Some(PlayerPiece::new(Player::White, PieceType::Rook))
    );

    // Rank 2
    assert_eq!(
        position.get_piece_debug("a2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("b2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("c2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("d2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );

    assert_eq!(
        position.get_piece_debug("e2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("f2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("g2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("h2"),
        Some(PlayerPiece::new(Player::White, PieceType::Pawn))
    );

    // Rank 3
    assert_eq!(position.get_piece_debug("a3"), None);
    assert_eq!(position.get_piece_debug("b3"), None);
    assert_eq!(position.get_piece_debug("c3"), None);
    assert_eq!(position.get_piece_debug("d3"), None);

    assert_eq!(position.get_piece_debug("e3"), None);
    assert_eq!(position.get_piece_debug("f3"), None);
    assert_eq!(position.get_piece_debug("g3"), None);
    assert_eq!(position.get_piece_debug("h3"), None);

    // Rank 4
    assert_eq!(position.get_piece_debug("a4"), None);
    assert_eq!(position.get_piece_debug("b4"), None);
    assert_eq!(position.get_piece_debug("c4"), None);
    assert_eq!(position.get_piece_debug("d4"), None);

    assert_eq!(position.get_piece_debug("e4"), None);
    assert_eq!(position.get_piece_debug("f4"), None);
    assert_eq!(position.get_piece_debug("g4"), None);
    assert_eq!(position.get_piece_debug("h4"), None);

    // Rank 5
    assert_eq!(position.get_piece_debug("a5"), None);
    assert_eq!(position.get_piece_debug("b5"), None);
    assert_eq!(position.get_piece_debug("c5"), None);
    assert_eq!(position.get_piece_debug("d5"), None);

    assert_eq!(position.get_piece_debug("e5"), None);
    assert_eq!(position.get_piece_debug("f5"), None);
    assert_eq!(position.get_piece_debug("g5"), None);
    assert_eq!(position.get_piece_debug("h5"), None);

    // Rank 6
    assert_eq!(position.get_piece_debug("a6"), None);
    assert_eq!(position.get_piece_debug("b6"), None);
    assert_eq!(position.get_piece_debug("c6"), None);
    assert_eq!(position.get_piece_debug("d6"), None);

    assert_eq!(position.get_piece_debug("e6"), None);
    assert_eq!(position.get_piece_debug("f6"), None);
    assert_eq!(position.get_piece_debug("g6"), None);
    assert_eq!(position.get_piece_debug("h6"), None);

    // Rank 7
    assert_eq!(
        position.get_piece_debug("a7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("b7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("c7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("d7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );

    assert_eq!(
        position.get_piece_debug("e7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("f7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("g7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );
    assert_eq!(
        position.get_piece_debug("h7"),
        Some(PlayerPiece::new(Player::Black, PieceType::Pawn))
    );

    // Rank 8
    assert_eq!(
        position.get_piece_debug("a8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Rook))
    );
    assert_eq!(
        position.get_piece_debug("b8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Knight))
    );
    assert_eq!(
        position.get_piece_debug("c8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Bishop))
    );
    assert_eq!(
        position.get_piece_debug("d8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Queen))
    );

    assert_eq!(
        position.get_piece_debug("e8"),
        Some(PlayerPiece::new(Player::Black, PieceType::King))
    );
    assert_eq!(
        position.get_piece_debug("f8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Bishop))
    );
    assert_eq!(
        position.get_piece_debug("g8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Knight))
    );
    assert_eq!(
        position.get_piece_debug("h8"),
        Some(PlayerPiece::new(Player::Black, PieceType::Rook))
    );
}

#[test]
fn start_has_20_moves() {
    let position = Position::default();

    let legal_moves = position.get_all_legal_moves();

    assert_eq!(legal_moves.len(), 20);
}

#[test]
fn white_is_starting_player() {
    let position = Position::default();

    assert_eq!(position.current_player(), Player::White);
}