use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use rand::Rng;

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

    // Board
    const SIZE: usize = 4;
    let mut board = Board::new(SIZE, SIZE);
    //board.set_word(&WordPos::new(0, 0, Dir::HOR, 4), "CIAO");
    //board.set_word(&WordPos::new(2, 0, Dir::VER, 4), "TEST");
    //board.set(0, 2, '#');

    // Create list of missing word positions
    let mut words_pos = board.get_words_pos();
    words_pos.sort_by(|a, b| Ord::cmp(&a.len, &b.len));

    // create map of word_pos -> intersecting word_pos
    let mut words_intersect: HashMap<&WordPos, Vec<&WordPos>> = HashMap::new();
    for word_pos in &words_pos {
        match word_pos.dir {
            Dir::HOR => {
                let wi: Vec<&WordPos> = words_pos.iter().filter(|wp|
                    wp.dir == Dir::VER &&
                    wp.y <= word_pos.y &&
                    wp.y + wp.len > word_pos.y).collect();
                words_intersect.insert(word_pos, wi);
            },
            Dir::VER => {
                let wi: Vec<&WordPos> = words_pos.iter().filter(|wp|
                        wp.dir == Dir::HOR &&
                        wp.x <= word_pos.x &&
                        wp.x + wp.len > word_pos.x).collect();
                words_intersect.insert(word_pos, wi);
            }
        }
    }

    // fill board
    let time_fill = SystemTime::now();
    fill_board(&mut board, &words_len, &words_pos, &words_intersect, &mut Vec::new());
    board.print();
    println!("Time to fill the board : {} ms", time_fill.elapsed().unwrap().as_millis());

    // print definitions
    print_definitions(&board, &words_pos, &json);
}


fn load_words(path: &str) -> serde_json::Value {
    let content: String = fs::read_to_string(path).expect("Unable to read text file");
    let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
    json
}


fn get_valid_words(words: &Vec<&str>, word_board: &str) -> usize {
    words
        .into_iter()
        .filter(|word| is_valid(word_board, word))
        .count()
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


fn fill_board<'a>(board: &mut Board, words_len: &HashMap<usize, Vec<&'a str>>, words_pos: &[WordPos],
                    words_intersect: &HashMap<&WordPos, Vec<&WordPos>>, used_words: &mut Vec<&'a str>) -> bool {
    
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
            board.set_word(current_word_pos, current_word);

            // check that exists at least one intersecting word for each letter of the current word
            let mut sol = true;
            for word_pos_intersect in words_intersect.get(current_word_pos).unwrap() {
                let word_board_intersect = board.get_word(word_pos_intersect);
                if get_valid_words(available_words, word_board_intersect.as_str()) == 0 {
                    sol = false;
                    break;
                }
            }
            
            if sol {
                used_words.push(&current_word);
                valid = fill_board(board, words_len, &words_pos[..words_pos.len() - 1], words_intersect, used_words);
                if valid {
                    break;
                }
                used_words.pop();
            }
        }

        if !valid {
            board.set_word(current_word_pos, current_word_board.as_str());
        }
    }
    else {
        valid = true;
    }
    valid
}


fn print_definitions(board: &Board, words_pos: &Vec<WordPos>, json: &serde_json::Value) {
    println!("Definizioni:");
    for word_pos in words_pos {
        let word = board.get_word(word_pos);
        let defs = json.get(word).unwrap().as_array().unwrap();

        let random_index = rand::thread_rng().gen_range(0..defs.len());
        println!("{:?}: {}", word_pos, defs.get(random_index).unwrap());
    }
}
