mod board;
mod trie;

use trie::TrieStruct;

fn main() {
    let mut trie = trie::TrieStruct::create();

    // C  A
    // N  E
    trie.insert("can".to_string());
    trie.insert("cane".to_string());
    trie.insert("ace".to_string());

    trie.is_word(&mut "ace".to_string());
    trie.is_word(&mut "can".to_string());
    let exists = trie.is_word(&mut "ace".to_string());
    let dne = trie.is_word(&mut "ane".to_string());

    // print if exists is a word
    // println!("ace is a word: {}", exists);
    // print if dne is a word
    // println!("can is a word: {}", dne);
    // create a fixed board
    // let board = board::generate_fixed_board();
    // print the board
    // println!("{:?}", board);

    // create a small board for development
    let small_board = board::generate_board_from_string("cane".to_string(), 2);

    // print the small board
    println!("{:?}", small_board);

    // print the trie for debugging
    // println!("{:?}", trie);

    // create a three by three board
    // let threexthree = board::generate_board_from_string("caneeibrt".to_string(), 3);

    // print three by three
    // println!("{:?}", threexthree);

    // given a board and the trie, find all the words
    let words = find_words(small_board, &mut trie);

    // print the words
    println!("{:?}", words);
}

pub fn find_words(board: Vec<Vec<char>>, trie: &mut TrieStruct) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    // let mut visited: Vec<Vec<bool>> = Vec::new();

    fn find_words_from_cell(
        board: Vec<Vec<char>>,
        row: usize,
        col: usize,
        current_prefix: &mut Vec<char>,
        trie: &mut TrieStruct,
        // visited: &mut Vec<Vec<bool>>,
        words: &mut Vec<String>,
    ) {
        let current_letter = board[row][col];
        // print the current letter
        println!("current letter: {}", current_letter);

        // create a new prefix by adding the current letter to the current prefix
        let mut new_prefix = current_prefix.clone();
        // add the current letter to the new prefix
        new_prefix.push(current_letter);

        // if the cell is out of bounds, return
        if row < 0 || row >= board.len() || col < 0 || col >= board[row].len() {
            return;
        }

        // if the cell has already been visited, return
        // if visited[row][col] {
        //     return;
        // }

        // transform the prefix into a string
        let mut prefix_string = String::new();
        for letter in new_prefix.clone() {
            prefix_string.push(letter);
        }

        println!("prefix string: {}", prefix_string.to_string());

        // if the trie does not contain the cell's value, return
        if !trie.is_prefix(&mut prefix_string) {
            println!("{} is not a prefix", prefix_string.to_string());
            return;
        }

        println!("{} is a prefix", prefix_string.to_string());

        // if the trie contains the cell's value, add it to the words
        if trie.is_word(&mut prefix_string) {
            println!("{} is a word", prefix_string.to_string());
            words.push(prefix_string.to_string());
        }

        // get the valid neighbors
        let neighbors = get_valid_neighbors(
            board.clone(),
            row,
            col,
            // visited
        );

        // print the neighbors
        println!("{:?}", neighbors);

        // iterate through the neighbors
        for neighbor in neighbors {
            // find all the words starting from this neighbor

            // check if the neighbor has already been visited and return if it has
            // if visited[neighbor.0][neighbor.1] {
            //     return;
            // }

            find_words_from_cell(
                board.clone(),
                neighbor.0,
                neighbor.1,
                &mut new_prefix,
                trie,
                // visited,
                words,
            );
        }
    }

    // create an empty char vector
    let mut empty_startng_prefix: Vec<char> = Vec::new();

    // iterate through each cell on the board
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            // create a new visited matrix
            // visited = vec![vec![false; board[row].len()]; board.len()];

            // find all the words starting from this cell
            find_words_from_cell(
                board.clone(),
                row,
                col,
                &mut empty_startng_prefix,
                trie,
                // &mut visited,
                &mut words,
            );
        }
    }

    words
}

fn get_valid_neighbors(
    board: Vec<Vec<char>>,
    row: usize,
    col: usize,
    // visited: &mut Vec<Vec<bool>>,
) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    // handle up
    if row > 0
    // && !visited[row - 1][col]
    {
        neighbors.push((row - 1, col));
    }

    // handle down
    if row < board.len() - 1
    // && !visited[row + 1][col]
    {
        neighbors.push((row + 1, col));
    }

    // handle left
    if col > 0
    // && !visited[row][col - 1]
    {
        neighbors.push((row, col - 1));
    }

    // handle right
    if col < board[row].len() - 1
    // && !visited[row][col + 1]
    {
        neighbors.push((row, col + 1));
    }

    // handle up left
    if row > 0 && col > 0
    // && !visited[row - 1][col - 1]
    {
        neighbors.push((row - 1, col - 1));
    }

    // handle up right
    if row > 0 && col < board[row].len() - 1
    // && !visited[row - 1][col + 1]
    {
        neighbors.push((row - 1, col + 1));
    }

    // handle down left
    if row < board.len() - 1 && col > 0
    // && !visited[row + 1][col - 1]
    {
        neighbors.push((row + 1, col - 1));
    }

    // handle down right
    if row < board.len() - 1 && col < board[row].len() - 1
    // && !visited[row + 1][col + 1]
    {
        neighbors.push((row + 1, col + 1));
    }

    neighbors
}

// Trie stucture
// const VAL = {
//     root_node: {
//         value: None,
//         is_final: false,
//         child_nodes: {
//             'c': {
//                 value: Some('c'),
//                 is_final: false,
//                 child_nodes: {

//                     'a': {
//                         value: Some('a'),
//                         is_final: false,
//                         child_nodes: {
//                         'n': {
//                             value: Some('n'),
//                             is_final: true,
//                             child_nodes: {
//                         'e': {
//                             value: Some('e'),
//                             is_final: true,
//                             child_nodes: {}
//                         }
//                     }
//                 }
//             }
//         }
//     }
// },
//                     'a': {
//                         value: Some('a'),
//                         is_final: false,
//                         child_nodes: {
//                         'c': {
//                             value: Some('c'),
//                             is_final: false,
//                             child_nodes: {
//                         'e': {
//                             value: Some('e'),
//                             is_final: true,
//                             child_nodes: {}
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
// }
