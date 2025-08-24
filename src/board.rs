pub struct Board {
    pub bitboards: [u64; 12], // bitboards for each piece
}

#[derive(Clone, Copy)]
pub enum Piece {
    PawnWhite,
    KnightWhite,
    BishopWhite,
    RookWhite,
    QueenWhite,
    KingWhite,
    PawnBlack,
    KnightBlack,
    BishopBlack,
    RookBlack,
    QueenBlack,
    KingBlack,
}

#[derive(Clone, Copy)]
pub enum PieceColorless {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub fn colorless_to_piece(piece: PieceColorless, white: bool) -> Piece {
    match piece {
        PieceColorless::Pawn if white => Piece::PawnWhite,
        PieceColorless::Knight if white => Piece::KnightWhite,
        PieceColorless::Bishop if white => Piece::BishopWhite,
        PieceColorless::Rook if white => Piece::RookWhite,
        PieceColorless::Queen if white => Piece::QueenWhite,
        PieceColorless::King if white => Piece::KingWhite,
        PieceColorless::Pawn => Piece::PawnBlack,
        PieceColorless::Knight => Piece::KnightBlack,
        PieceColorless::Bishop => Piece::BishopBlack,
        PieceColorless::Rook => Piece::RookBlack,
        PieceColorless::Queen => Piece::QueenBlack,
        PieceColorless::King => Piece::KingBlack,
    }
}

pub fn piece_to_colorless(piece: Piece) -> PieceColorless {
    match piece {
        Piece::PawnWhite => PieceColorless::Pawn,
        Piece::KnightWhite => PieceColorless::Knight,
        Piece::BishopWhite => PieceColorless::Bishop,
        Piece::RookWhite => PieceColorless::Rook,
        Piece::QueenWhite => PieceColorless::Queen,
        Piece::KingWhite => PieceColorless::King,
        Piece::PawnBlack => PieceColorless::Pawn,
        Piece::KnightBlack => PieceColorless::Knight,
        Piece::BishopBlack => PieceColorless::Bishop,
        Piece::RookBlack => PieceColorless::Rook,
        Piece::QueenBlack => PieceColorless::Queen,
        Piece::KingBlack => PieceColorless::King,
    }
}

//TODO piece -> colorless and the other way round
