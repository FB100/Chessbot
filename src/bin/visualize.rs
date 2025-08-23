use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number|0xHEX>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];

    // allow "0x" hex parsing or decimal
    let num = if let Some(stripped) = input.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        input.parse::<u64>()
    };

    match num {
        Ok(bb) => visualize_bitboard(bb),
        Err(_) => {
            eprintln!("Error: '{}' is not a valid bitboard", input);
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
