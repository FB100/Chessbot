use std::collections::hash_map::Entry;
use chess_bot::board::Piece;
use rand::RngCore;
use std::fs::File;
use std::io::{Error, Write};

fn main() {
    generate_initial_magic_files()
}

#[derive(Copy, Clone)]
struct Magic {
    magic: u64,
    mask: u64,
    shift: u8,
    offset: usize,
}

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

        // Masks, don't have to expand to the Edge, Movegen does
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

fn movegen_naive(square: &u8, piece: Piece, bit_board: &u64) -> Vec<u64> {
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
    let moves: Vec<u64> = generate_occupancies(create_slider_movement_mask(square, piece))
        .iter()
        .map(|&_occ| move_bitboard)
        .collect();
    moves
}

fn generate_occupancies(mask: u64) -> Vec<u64> {
    let mut bit_positions = Vec::new();
    for i in 0..64 {
        if (mask >> i) & 1 != 0 {
            bit_positions.push(i);
        }
    }

    let num_bits = bit_positions.len();
    let mut occupancies = Vec::with_capacity(1 << num_bits);

    for index in 0..(1 << num_bits) {
        let mut occ = 0u64;
        for (j, &pos) in bit_positions.iter().enumerate() {
            if (index >> j) & 1 != 0 {
                occ |= 1u64 << pos;
            }
        }
        occupancies.push(occ);
    }

    occupancies
}

fn sparse_random_u64() -> u64 {
    let mut rng = rand::rng();
    let candidate = rng.next_u64() & rng.next_u64() & rng.next_u64();
    if candidate.count_ones() < 4 || candidate.count_ones() > 10 {
        sparse_random_u64() // Try again
    } else {
        candidate
    }
}

fn write_magic_to_file(magics: &[Magic; 64], piece: Piece, file: &mut File) -> Result<(), Error> {
    writeln!(
        file,
        "#[derive(Copy, Clone)] struct Magic {{ magic: u64, mask: u64, shift: u8, offset: usize,}}"
    )?;
    match piece {
        Piece::BishopWhite => {
            writeln!(file, "const MAGICS_B: [Magic; 64] = [")?;
        }
        Piece::RookWhite => {
            writeln!(file, "const MAGICS_R: [Magic; 64] = [")?;
        }
        Piece::BishopBlack => {
            writeln!(file, "const MAGICS_B: [Magic; 64] = [")?;
        }
        Piece::RookBlack => {
            writeln!(file, "const MAGICS_R: [Magic; 64] = [")?;
        }
        _ => panic!("write_magic_to_file can only handle bishops and rooks."),
    }

    for m in magics.iter() {
        writeln!(
            file,
            "    Magic {{ magic: 0x{:016x}, mask: 0x{:016x}, shift: {}, offset: {} }},",
            m.magic, m.mask, m.shift, m.offset
        )?;
    }
    writeln!(file, "];")?;
    Ok(())
}

fn generate_initial_magic_files(){
    let magic = Magic {
        magic: 0,
        mask: 0,
        shift: 0,
        offset: 0,
    };
    let magic_array: [Magic; 64] = [magic.clone(); 64];
    let mut file_bishop = File::create("src/constants/magics_bishop.rs").unwrap();
    let mut file_rook = File::create("src/constants/magics_rook.rs").unwrap();

    write_magic_to_file(&magic_array, Piece::BishopBlack, &mut file_bishop)
        .expect("TODO: panic message");
    write_magic_to_file(&magic_array, Piece::RookBlack, &mut file_rook)
        .expect("TODO: panic message");
}

fn find_magic(piece: Piece, mask: u64, square: &u8, min_relevant_bits: &u8) -> Option<Magic> {
    let mut magic = Magic {
        magic: sparse_random_u64(),
        mask,
        shift: 64-min_relevant_bits,
        offset: 0,
    };

    let mut table = std::collections::HashMap::new();
    let occupancies = generate_occupancies(mask);

    for i in *min_relevant_bits..64 {
        for (_j, &occ) in occupancies.iter().enumerate() {
            let index = ((occ.wrapping_mul(magic.magic)) >> magic.shift) as usize;
            let possible_move = movegen_naive(square, piece, &occ);

            if let Some(existing) = table.get(&index) {
                if *existing != possible_move {
                    return None;
                }
            } else {
                table.insert(index, possible_move);
            }
        }
        magic.shift = 64-i;
    }
    if(magic.shift < 64-min_relevant_bits) {
        return Some(magic);
    }
    None
}

fn find_all_magics() {


}
