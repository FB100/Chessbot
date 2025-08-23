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

//TODO piece -> colorless and the other way round
