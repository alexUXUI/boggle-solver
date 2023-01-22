use crate::TrieStruct;

pub fn find_words(board: Vec<Vec<char>>, trie: &mut TrieStruct) -> Vec<String> {
    let mut found_words: Vec<String> = Vec::new();

    fn find_words_from_cell(
        board: Vec<Vec<char>>,
        row: usize,
        col: usize,
        current_prefix: &mut Vec<char>,
        trie: &mut TrieStruct,
        words: &mut Vec<String>,
    ) {
        // get the current letter
        let current_letter: char = board[row][col];

        // generate a new prefix by cloning the current prefix and adding the current letter
        let mut new_prefix: Vec<char> = current_prefix.clone();
        new_prefix.push(current_letter);

        // transform the prefix into a string using the collect method
        let mut prefix_string: String = new_prefix.iter().collect();

        // if the trie does not contain the prefix string return
        if !trie.is_prefix(&mut prefix_string) {
            return;
        }

        // if the prefix string is a word add it to the words vector
        if trie.is_word(&mut prefix_string) {
            words.push(prefix_string.to_string());
        }

        // get cell neighbors
        let neighbors = get_valid_neighbors(board.clone(), row, col);

        // iterate through the neighbors
        for neighbor in neighbors {
            // Depth first search
            find_words_from_cell(
                board.clone(),
                neighbor.0,
                neighbor.1,
                &mut new_prefix,
                trie,
                words,
            );
        }
    }

    // create an empty prefix char vector to hold the current prefix
    let mut empty_startng_prefix: Vec<char> = Vec::new();

    // iterate through each cell on the board
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            // find words for each cell
            find_words_from_cell(
                board.clone(),
                row,
                col,
                &mut empty_startng_prefix,
                trie,
                &mut found_words,
            );
        }
    }

    found_words
}

fn get_valid_neighbors(board: Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    // handle up
    if row > 0 {
        neighbors.push((row - 1, col));
    }

    // handle down
    if row < board.len() - 1 {
        neighbors.push((row + 1, col));
    }

    // handle left
    if col > 0 {
        neighbors.push((row, col - 1));
    }

    // handle right
    if col < board[row].len() - 1 {
        neighbors.push((row, col + 1));
    }

    // handle up left
    if row > 0 && col > 0 {
        neighbors.push((row - 1, col - 1));
    }

    // handle up right
    if row > 0 && col < board[row].len() - 1 {
        neighbors.push((row - 1, col + 1));
    }

    // handle down left
    if row < board.len() - 1 && col > 0 {
        neighbors.push((row + 1, col - 1));
    }

    // handle down right
    if row < board.len() - 1 && col < board[row].len() - 1 {
        neighbors.push((row + 1, col + 1));
    }

    neighbors
}
