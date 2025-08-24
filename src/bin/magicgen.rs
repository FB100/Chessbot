use chess_bot::board::PieceColorless;
use chess_bot::constants::Magic;
use chess_bot::constants::magics_bishop::MAGICS_B;
use chess_bot::constants::magics_rook::MAGICS_R;
use rand::RngCore;
use std::env;
use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => print_wrong_input(&args[0]),
        2 => {
            if args[1] == "h" || args[1] == "help" {
                eprintln!("Initialise new magics: {} < i | init > [filename]", args[0]);
                eprintln!(
                    "Refine existing magics: {} <bishop: bool> [filename]",
                    args[0]
                );
                std::process::exit(0);
            } else if args[1] == "i" || args[1] == "init" {
                generate_initial_magic_files()
            } else {
                print_wrong_input(&args[0])
            }
        }
        3 => {}
        _ => print_wrong_input(&args[0]),
    }
}

fn print_wrong_input(command: &str) {
    eprintln!("For help: {} < h | help >", command);
    std::process::exit(1);
}

fn helper_mask_and_movegen(
    movegen: bool,
    square: &usize,
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

        while (r > minsize || (rank == 0 && r >= 0))
            && (r < maxsize || (rank == 7 && r <= 7))
            && (f > minsize || (file == 0 && f >= 0))
            && (f < maxsize || (file == 7 && f <= 7))
        {
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
fn create_slider_movement_mask(square: &usize, piece: PieceColorless) -> u64 {
    let mask;

    let directions_bishop = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let directions_rook = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    match piece {
        PieceColorless::Bishop => {
            mask = helper_mask_and_movegen(false, square, &0, directions_bishop)
        }
        PieceColorless::Rook => mask = helper_mask_and_movegen(false, square, &0, directions_rook),
        PieceColorless::Queen => {
            mask = helper_mask_and_movegen(false, square, &0, directions_bishop)
                | helper_mask_and_movegen(false, square, &0, directions_rook)
        }
        _ => mask = 0,
    }

    mask
}

fn movegen_naive(square: &usize, piece: PieceColorless, bit_board: &u64) -> Vec<u64> {
    let move_bitboard;

    let directions_bishop = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let directions_rook = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    match piece {
        PieceColorless::Bishop => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_bishop)
        }
        PieceColorless::Rook => {
            move_bitboard = helper_mask_and_movegen(true, square, bit_board, directions_rook)
        }
        PieceColorless::Queen => {
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

fn write_magic_to_file(
    magics: &[Magic; 64],
    piece: PieceColorless,
    file: &mut File,
) -> Result<(), Error> {
    writeln!(
        file,
        "#[derive(Copy, Clone)] pub struct Magic {{pub magic: u64,pub mask: u64,pub shift: u8,pub offset: usize,}}"
    )?;
    match piece {
        PieceColorless::Bishop => {
            writeln!(file, "pub const MAGICS_B: [Magic; 64] = [")?;
        }
        PieceColorless::Rook => {
            writeln!(file, "pub const MAGICS_R: [Magic; 64] = [")?;
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

fn generate_initial_magic_files() {
    let magic = Magic {
        magic: 0,
        mask: 0,
        shift: 0,
        offset: 0,
    };
    let mut magic_array: [Magic; 64] = [magic.clone(); 64];

    for i in 0..64 {
        magic_array[i].mask = create_slider_movement_mask(&i, PieceColorless::Bishop);
    }
    let mut file_bishop = File::create("src/constants/magics_bishop.rs").unwrap();
    write_magic_to_file(&magic_array, PieceColorless::Bishop, &mut file_bishop)
        .expect("TODO: panic message");

    for i in 0..64 {
        magic_array[i].mask = create_slider_movement_mask(&i, PieceColorless::Rook);
    }
    let mut file_rook = File::create("src/constants/magics_rook.rs").unwrap();
    write_magic_to_file(&magic_array, PieceColorless::Rook, &mut file_rook)
        .expect("TODO: panic message");
}

fn find_magic(piece: PieceColorless, mask: u64, square: &usize, max_shift: &u8) -> Option<Magic> {
    let mut magic = Magic {
        magic: sparse_random_u64(),
        mask,
        shift: *max_shift,
        offset: 0,
    };

    let mut table = std::collections::HashMap::new();
    let occupancies = generate_occupancies(mask);

    for i in (64 - max_shift)..64 {
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
        magic.shift = 64 - i;
    }
    if (magic.shift > *max_shift) {
        return Some(magic);
    }
    None
}

fn find_all_magics(piece: PieceColorless, file: &mut File) {
    let mut temp_magics;
    match piece {
        PieceColorless::Bishop => temp_magics = MAGICS_B.clone(),
        PieceColorless::Rook => temp_magics = MAGICS_R.clone(),
        _ => panic!(""),
    }

    for _runs in 0..10 {
        for square in 0..64 {
            let possible_magic = find_magic(
                piece,
                temp_magics[square].mask,
                &square,
                &temp_magics[square].shift,
            );
            if let Some(magic) = possible_magic {
                temp_magics[square] = magic;
            }
        }
        write_magic_to_file(&temp_magics, piece, file).unwrap();
    }
}
