mod board;
mod solver;
mod trie;

use solver::find_words;
use trie::TrieStruct;

fn main() {
    // Create a trie to store the dictionary
    let mut trie: TrieStruct = trie::TrieStruct::create();

    // Insert words into the trie dictionary
    trie.insert("bloat".to_string());
    trie.insert("inmate".to_string());
    trie.insert("anime".to_string());
    trie.insert("laminate".to_string());

    // Generate a fixed board
    let fixed_board: Vec<Vec<char>> = board::generate_fixed_board();

    // Find the words on the board
    let words: Vec<String> = find_words(fixed_board, &mut trie);

    // print the words
    println!("{:?}", words);
}
