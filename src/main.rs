mod board;
mod solver;
mod trie;

use solver::find_words;
use trie::TrieStruct;

fn main() {
    // Create a trie to store the dictionary
    let mut trie = trie::TrieStruct::create();

    // Insert words into the trie dictionary
    trie.insert("bloat".to_string());
    trie.insert("inmate".to_string());
    trie.insert("anime".to_string());
    trie.insert("laminate".to_string());

    // Generate a fixed board
    let fixed_board = board::generate_fixed_board();

    let board_from_string = board::generate_board_from_string("cane".to_string(), 2);

    // Find the words on the board
    let words = find_words(fixed_board, &mut trie);

    // print the words
    println!("{:?}", words);
}
