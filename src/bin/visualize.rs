use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    match args[1].parse::<u64>() {
        Ok(num) => visualize_bitboard(num),
        Err(_) => {
            eprintln!("Error: '{}' is not a valid bitboard", args[1]);
            std::process::exit(1);
        }
    }
}

pub fn visualize_bitboard(bb: u64) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let sq = rank * 8 + file;
            let bit = 1u64 << sq;
            if bb & bit != 0 {
                print!("♟ ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
    println!("  a b c d e f g h");
}


