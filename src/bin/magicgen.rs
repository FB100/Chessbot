use chess_bot::board::Piece;
use rand::random;

fn main() {}

fn create_slider_movement_mask(square: &u8, piece: Piece) -> u64 {
    let mask;

    fn create_mask_helper(square: &u8, directions: [(i8, i8); 4]) -> u64 {
        let rank = (square / 8) as i8;
        let file = (square % 8) as i8;

        let mut mask: u64 = 0;

        for (x, y) in directions {
            let mut r = rank + x;
            let mut f = file + y;
            while r > 0 && r < 7 && f > 0 && f < 7 {
                mask |= 1u64 << (r * 8 + f);
                r += x;
                f += y;
            }
        }
        mask
    }

    let directions_bishop = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let directions_rook = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    match piece {
        Piece::BishopWhite => mask = create_mask_helper(square, directions_bishop),
        Piece::BishopBlack => mask = create_mask_helper(square, directions_bishop),
        Piece::RookWhite => mask = create_mask_helper(square, directions_rook),
        Piece::RookBlack => mask = create_mask_helper(square, directions_rook),
        Piece::QueenWhite => {
            mask = create_mask_helper(square, directions_bishop)
                | create_mask_helper(square, directions_rook)
        }
        Piece::QueenBlack => {
            mask = create_mask_helper(square, directions_bishop)
                | create_mask_helper(square, directions_rook)
        }
        _ => mask = 0,
    }

    mask
}

fn movegen_naive(square: &u8, mask: &u64, bit_board: &u64) -> u64 {
    //Just for Compiler to stop complaining, while I'm working:
    0
}

fn find_magic(bit_board: &u64) -> u64 {
    movegen_naive(&0, &0, &0);
    0
}
