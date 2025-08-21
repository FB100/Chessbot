use chess_bot::board::Piece;

fn main() {}

fn helper_mask_and_movegen(
    movegen: bool,
    square: &u8,
    bit_board: &u64,
    directions: [(i8, i8); 4],
) -> u64 {
    let rank = (square / 8) as i8;
    let file = (square % 8) as i8;

    let mut move_bitboard: u64 = 0;

    for (x, y) in directions {
        let mut r = rank + x;
        let mut f = file + y;

        let mut maxsize = 7;
        let mut minsize = 0;

        if movegen {
            maxsize = 8;
            minsize = -1;
        }

        while r > minsize && r < maxsize && f > minsize && f < maxsize {
            move_bitboard |= 1u64 << (r * 8 + f);
            if ((1 << r * 8 + f) & bit_board == 1) && movegen {
                break;
            }
            r += x;
            f += y;
        }
    }
    move_bitboard
}
fn create_slider_movement_mask(square: &u8, piece: Piece) -> u64 {
    let mask;

    let directions_bishop = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let directions_rook = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    match piece {
        Piece::BishopWhite => mask = helper_mask_and_movegen(false, square, &0, directions_bishop),
        Piece::BishopBlack => mask = helper_mask_and_movegen(false, square, &0, directions_bishop),
        Piece::RookWhite => mask = helper_mask_and_movegen(false, square, &0, directions_rook),
        Piece::RookBlack => mask = helper_mask_and_movegen(false, square, &0, directions_rook),
        Piece::QueenWhite => {
            mask = helper_mask_and_movegen(false, square, &0, directions_bishop)
                | helper_mask_and_movegen(false, square, &0, directions_rook)
        }
        Piece::QueenBlack => {
            mask = helper_mask_and_movegen(false, square, &0, directions_bishop)
                | helper_mask_and_movegen(false, square, &0, directions_rook)
        }
        _ => mask = 0,
    }

    mask
}

fn movegen_naive(square: &u8, piece: Piece, bit_board: &u64) -> u64 {
    let move_bitboard;

    let directions_bishop = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let directions_rook = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    match piece {
        Piece::BishopWhite => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_bishop)
        }
        Piece::BishopBlack => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_bishop)
        }
        Piece::RookWhite => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_rook)
        }
        Piece::RookBlack => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_rook)
        }
        Piece::QueenWhite => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_bishop)
                | helper_mask_and_movegen(true, square, bit_board, directions_rook)
        }
        Piece::QueenBlack => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_bishop)
                | helper_mask_and_movegen(true, square, bit_board, directions_rook)
        }
        _ => move_bitboard = 0,
    }

    move_bitboard
}

fn find_magic(bit_board: &u64) -> u64 {
    movegen_naive(&0, Piece::BishopBlack, &0);
    0
}
