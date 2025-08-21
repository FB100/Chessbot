use rand::random;
use chess_bot::board::Piece;

fn main() {
    let rand = random::<u64> as u64;
}

fn movegen_naive(square: &u8,mask: &u64, bit_board: &u64) -> u64 {




    //Just for Compiler to stop complaining, while I'm working:
    0
}

fn create_slider_movement_mask(square: &u8, piece: Piece) -> u64 {

    match piece {
        Piece::BishopWhite => {}
        Piece::RookWhite => {}
        Piece::QueenWhite => {}
        Piece::BishopBlack => {}
        Piece::RookBlack => {}
        Piece::QueenBlack => {}
        _ => return 0,
    }


    //Just for Compiler to stop complaining, while I'm working:
    0
}

fn find_magic(bit_board: &u64) -> u64 {
    movegen_naive(&0,&0, &0);
    0
}
