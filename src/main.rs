use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
//use rand::Rng;

use utils::{Board, WordPos, Dir};

mod utils;

fn main() {
    println!("Crosswords Generator v0.1");
    //let random_index = rand::thread_rng().gen_range(0..100);
    
    // Load json words and definitions
    let time_json = SystemTime::now();
    let json = load_words("./data/words.txt");
    println!("Time to read and parse json: {} ms", time_json.elapsed().unwrap().as_millis());
    //println!("Words Text: {}", json.get("AA").unwrap());

    // Create map (len -> words)
    let time_maplen = SystemTime::now();
    let mut words_len: HashMap<usize, Vec<&str>> = HashMap::new();
    for (key, _) in json.as_object().unwrap() {
        if words_len.contains_key(&key.len()) {
            words_len.get_mut(&key.len()).unwrap().push(key);
        } else {
            words_len.insert(key.len(), vec![key]);
        }
    }
    println!("Time to create the map (len->words): {} ms", time_maplen.elapsed().unwrap().as_millis());
    //println!("MAP LEN-WORDS: {:?}\n", words_len.get(&6));

    // Board
    let mut board = Board::new(4, 4);
    //board.set_word(&WordPos::new(0, 0, Dir::HOR, 4), "CIAO");
    //board.set_word(&WordPos::new(2, 0, Dir::VER, 4), "TEST");
    //board.set(0, 2, '#');
    //board.print();

    // Create list of missing word positions
    let mut words_pos = board.get_words_pos();
    words_pos.sort_by(|a, b| Ord::cmp(&a.len, &b.len));
    println!("{:?}", words_pos);

    //println!("is_valid: {:?}", is_valid("A  TO", "BALZO"));
    //println!("is_valid: {:?}", is_valid("A  TO", "AIUTO"));
    //println!("is_valid: {:?}", is_valid("A  TO", "AMPIO"));

    // fill board
    fill_board(&mut board, &words_len, &words_pos[..], &mut Vec::new());
    board.print();

}

fn load_words(path: &str) -> serde_json::Value {
    let content: String = fs::read_to_string(path).expect("Unable to read text file");
    let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
    json
}

fn is_valid(word_board: &str, word: &str) -> bool {
    let mut word_chars = word.chars();
    for c in word_board.chars() {
        let c2 = word_chars.next().unwrap();
        if c == ' ' {
            continue;
        }
        if c != c2 {
            return false;
        }
    }
    true
}

fn fill_board<'a>(board: &mut Board, words_len: &HashMap<usize, Vec<&'a str>>, words_pos: &[WordPos], used_words: &mut Vec<&'a str>) -> bool {
    let mut valid = false;
    if words_pos.len() > 0 {
        let current_word_pos = words_pos.last().unwrap();
        let available_words = words_len.get(&current_word_pos.len).unwrap();

        let current_word_board = board.get_word(current_word_pos);
        let valid_words = available_words
            .into_iter()
            .filter(|word| is_valid(current_word_board.as_str(), word));
        
        for current_word in valid_words {
            if used_words.contains(current_word) {
                continue;
            }
            used_words.push(&current_word);
            board.set_word(current_word_pos, current_word);
            valid = fill_board(board, words_len, &words_pos[..words_pos.len() - 1], used_words);
            if valid {
                break;
            }
            used_words.pop();
        }

        board.set_word(current_word_pos, current_word_board.as_str());
    }
    else {
        valid = true;
    }
    valid
}

