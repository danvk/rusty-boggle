mod boggler;
mod trie;
mod util;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Expected two arguments, got {}: {:?}", args.len(), args);
    }

    let dict_path = args.get(1).unwrap();
    let board = args.get(2).unwrap();
    let mut dict = boggler::load_dictionary(dict_path).unwrap();

    println!(
        "Loaded {} words into {} nodes from {}",
        dict.size(),
        dict.num_nodes(),
        dict_path
    );

    let mut boggler = boggler::Boggler::new();
    boggler.parse_board(board).unwrap();
    let score = boggler.score(&mut dict);
    println!("{}: {}", boggler, score);

    let prime: u32 = (1 << 20) - 3;
    let mut total_score: u32 = 0;
    let mut hash: u32 = 1234;
    let mut num_boards = 0;
    let bases: [&str; 2] = ["abcdefghijklmnop", "catdlinemaropets"];
    let start = Instant::now();

    for rep in 0..1 {
        hash = 1234;
        for base in bases.iter() {
            boggler.parse_board(base).unwrap();
            for y1 in 0..4 {
                for y2 in 0..4 {
                    for c1 in 0..26 {
                        boggler.set_cell(1, y1, c1);
                        for c2 in 0..26 {
                            boggler.set_cell(2, y2, c2);
                            boggler.parse_board(&boggler.to_string()).unwrap();
                            let score = boggler.score(&mut dict);
                            hash *= 123 + score;
                            hash = hash % prime;
                            total_score += score;
                            num_boards += 1;
                            // println!("{}{},{}/{} {} {} -> {}", y1, y2, c1, c2, boggler, score, total_score)
                        }
                    }
                }
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f32();
    // if hash != 0x000C1D3D {
    //     panic!("Hash mismatch, expected 0x000C1D3D but got {}", hash);
    // }

    println!(
        "Total score: {} = {} pts/bd",
        total_score,
        (total_score as f32) / (num_boards as f32),
    );
    println!("Score hash: {:x}", hash);
    println!(
        "Evaluated {} boards in {} seconds = {} bds/sec",
        num_boards,
        elapsed,
        1.0 * (num_boards as f32) / elapsed
    );
}
