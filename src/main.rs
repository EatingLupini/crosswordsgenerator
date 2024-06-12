use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use rand::Rng;

use utils::{Board, WordPos, Dir};

mod utils;

fn main() {
    println!("Crosswords Generator v0.1");

    // Load json words and definitions
    let time_json = SystemTime::now();
    let json = load_words("./data/words.txt");
    println!("Time to read and parse json: {} ms", time_json.elapsed().unwrap().as_millis());

    // Create map (len -> words)
    let time_maplen = SystemTime::now();
    let mut words_len: HashMap<usize, Vec<&str>> = HashMap::new();
    for (key, _) in json.as_object().unwrap() {
        let len = key.len();
        words_len.entry(len).or_insert_with(Vec::new).push(key);
    }
    println!("Time to create the map (len->words): {} ms", time_maplen.elapsed().unwrap().as_millis());


    // Board
    const SIZE: usize = 6;
    let mut board = Board::new(SIZE, SIZE);
    // board.set(0, 0, '#');
    // board.set(1, 1, '#');
    // board.set(2, 2, '#');
    // board.set(3, 3, '#');
    // board.set(4, 4, '#');
    // board.set(5, 5, '#');
    // board.set(6, 6, '#');
    

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
    let mut visited_nodes: usize = 0;
    fill_board(&mut board, &words_len, &words_pos, &words_intersect, &mut Vec::new(), &mut HashMap::new(), &mut visited_nodes);
    board.print();
    println!("Visited nodes: {}", visited_nodes);
    println!("Time to fill the board : {} ms", time_fill.elapsed().unwrap().as_millis());

    // print definitions
    print_definitions(&board, &words_pos, &json);
}


fn load_words(path: &str) -> serde_json::Value {
    let content: String = fs::read_to_string(path).expect("Unable to read text file");
    let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
    json
}


fn fill_board<'a>(board: &mut Board, words_len: &'a HashMap<usize, Vec<&'a str>>, words_pos: &[WordPos],
                    words_intersect: &HashMap<&WordPos, Vec<&WordPos>>, words_used: &mut Vec<&'a str>,
                    words_map_cache: &mut HashMap<String, Vec<&'a str>>, visited_nodes: &mut usize) -> bool {
    //board.print();
    if words_pos.is_empty() {
        return true;
    }

    let mut valid = false;
    let current_word_pos = words_pos.last().unwrap();
    let current_word_board = board.get_word(current_word_pos);
    let valid_words = get_valid_words(words_len.get(&current_word_pos.len).unwrap(), current_word_board.as_str());
    //let valid_words = get_valid_words_cache(words_map_cache, words_len.get(&current_word_pos.len).unwrap(), &current_word_board);

    for current_word in valid_words {
        if words_used.contains(&current_word) {
            continue;
        }
        board.set_word(current_word_pos, current_word);
        *visited_nodes += 1;

        // check that exists at least one intersecting word for each letter of the current word
        let mut sol = true;
        for word_pos_intersect in words_intersect.get(current_word_pos).unwrap() {
            let word_board_intersect = board.get_word(word_pos_intersect);
            let words_intersect_num: usize;

            let valid_words_cached = words_map_cache.get(&word_board_intersect);
            if valid_words_cached.is_some() {
                words_intersect_num = valid_words_cached.unwrap().len();
            }
            else {
                let valid_words_intersect = get_valid_words(words_len.get(&word_pos_intersect.len).unwrap(), word_board_intersect.as_str());
                words_intersect_num = valid_words_intersect.len();

                words_map_cache.insert(word_board_intersect, valid_words_intersect);
            }

            if words_intersect_num == 0 {
                sol = false;
                break;  
            }
        }
        
        if sol {
            words_used.push(&current_word);
            valid = fill_board(board, words_len, &words_pos[..words_pos.len() - 1], words_intersect, words_used, words_map_cache, visited_nodes);
            if valid {
                break;
            }
            words_used.pop();
        }
    }

    if !valid {
        board.set_word(current_word_pos, current_word_board.as_str());
    }

    valid
}


fn get_valid_words_cache<'a>(words_map_cache: &'a mut HashMap<String, Vec<&'a str>>, available_words: &'a Vec<&str>, word_board: &String) -> &'a Vec<&'a str> {
    let entry = words_map_cache.entry(word_board.clone()).or_insert_with(|| {
        get_valid_words(available_words, word_board.as_str())
    });
    
    entry
}


fn get_valid_words<'a>(words: &'a [&str], word_board: &str) -> Vec<&'a str> {
    words
        .iter()
        .filter(|&&word| is_valid(word_board, word))
        .cloned()
        .collect()
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


fn print_definitions(board: &Board, words_pos: &Vec<WordPos>, json: &serde_json::Value) {
    println!("\nDefs:");
    for word_pos in words_pos {
        let word = board.get_word(word_pos);
        let defs = json.get(word).unwrap().as_array().unwrap();

        let random_index = rand::thread_rng().gen_range(0..defs.len());
        println!("{:?}: {}", word_pos, defs.get(random_index).unwrap());
    }
}
