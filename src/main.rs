mod boggler;
mod trie;
mod util;
use std::env;

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
}
