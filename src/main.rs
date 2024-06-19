use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::SystemTime;

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

use clap::{command, Arg};

use utils::{Board, WordPos, Dir};

mod utils;

static EMPTY_VEC: Vec<&str> = Vec::new();

#[derive(Debug, PartialEq, Eq)]
pub enum Fs {
    FWD,
    LOOP,
    BWD
}


fn main() {
    // Check arguments
    let args = command!().about("Crosswords Generator v0.1\nSmall application to fill a provided Crossword Board.")
    .arg(
        Arg::new("size").short('s').long("size")
        .help("Size of the board.")
        .num_args(2)
        .value_parser(clap::value_parser!(usize))
        .default_values(["4", "4"])
        .conflicts_with("board")
    )
    .arg(
        Arg::new("board").short('b').long("board")
        .help("Path to a board.")
    )
    .arg(
        Arg::new("shuffle").short('x').long("shuffle")
        .help("Shuffle the words before filling the board.")
        .num_args(0..=1)
        .value_parser(["true", "false"])
        .default_value("false")
        .default_missing_value("true")
    )
    .arg(
        Arg::new("repeat-words").short('r').long("repeat-words")
        .help("Allow words to be repeated.")
        .num_args(0..=1)
        .value_parser(["true", "false"])
        .default_value("false")
        .default_missing_value("true")   
    )
    .get_matches();


    // CROSSWORDS GENERATOR
    println!("Crosswords Generator v0.1");

    // Settings
    let size: Vec<usize> = args.get_many("size").unwrap().copied().collect();
    let board_w = *size.get(0).unwrap();
    let board_h = *size.get(1).unwrap();
    let board_path = args.get_one::<String>("board");
    let shuffle = args.get_one::<String>("shuffle").unwrap()
        .parse::<bool>().unwrap_or_else(|e| panic!("Argument 'shuffle' error: {}", e));
    let rep_words = args.get_one::<String>("repeat-words").unwrap()
        .parse::<bool>().unwrap_or_else(|e| panic!("Argument 'repeat-words' error: {}", e));
    
    println!("\nSettings:");
    println!("- size: {:?}", size);
    println!("- board: {:?}", board_path);
    println!("- shuffle: {}", shuffle);
    println!("- repeat-words: {}", rep_words);
    println!();


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
    // Randomize words
    if shuffle {
        for words in words_len.values_mut() {
            words.shuffle(&mut thread_rng());
        }
    }
    println!("Time to create the map (len->words): {} ms", time_maplen.elapsed().unwrap().as_millis());
    

    // Board
    let mut board = Board::new(board_w, board_h);
    // board.set(0, 0, '#');

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
    let mode: u8 = 0;
    let mut found = false;
    let mut visited_nodes: usize = 0;

    // recursive
    if mode == 0 {
        found = fill_board(&mut board, &words_len, &words_pos, &words_intersect,
            &mut HashSet::with_capacity(words_pos.len()),
            &mut HashMap::new(), &mut visited_nodes,
            rep_words);
    }

    // iter
    else if mode == 1 {
        let mut wp = words_pos.clone();
        found = fill_board_iter(&mut board, &words_len, &mut wp, &words_intersect,
                            &mut Vec::with_capacity(words_pos.len()),
                            &mut HashMap::new(), &mut visited_nodes,
                            rep_words);
    }

    // parallel
    else if mode == 2 {
        // Make a vector to hold the children which are spawned.
        /*static NPROC: u8 = 4;
        let mut children = vec![];
        let words_map_cache: Arc<Mutex<HashMap<String, Vec<&str>>>> = Arc::new(Mutex::new(HashMap::new()));

        for i in 0..NPROC {
            let mut t_board = board.clone();
            let t_words_len: HashMap<usize, Vec<&str>> = HashMap::new(); //words_len.clone();
            let mut t_words_pos: Vec<WordPos> = words_pos.clone();
            let t_words_interesect = words_intersect.clone();
            let mut t_words_used: Vec<&str> = Vec::with_capacity(words_pos.len());

            children.push(thread::spawn(move || {
                let found = fill_board_iter(&mut t_board, &t_words_len, &mut t_words_pos, &t_words_interesect,
                    &mut t_words_used,
                    &mut HashMap::new(), &mut visited_nodes,
                    rep_words);

                println!("this is thread number {}", i);
                }
            )); 
        }

        for child in children {
            let _ = child.join();
        }*/
    }
        
    if found {
        board.print();
        println!("Time to fill the board: {} ms", time_fill.elapsed().unwrap().as_millis());

        // print definitions
        print_definitions(&board, &words_pos, &json);
    }
    else {
        println!("No solution found in: {} ms", time_fill.elapsed().unwrap().as_millis());
    }

    println!("\nSTATS");
    println!("Visited nodes: {}", visited_nodes);
}


fn load_words(path: &str) -> serde_json::Value {
    let content: String = fs::read_to_string(path).expect("Unable to read text file");
    let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
    json
}


fn fill_board<'a>(board: &mut Board, words_len: &'a HashMap<usize, Vec<&'a str>>, words_pos: &[WordPos],
                    words_intersect: &HashMap<&WordPos, Vec<&WordPos>>, words_used: &mut HashSet<&'a str>,
                    words_map_cache: &mut HashMap<String, Vec<&'a str>>, visited_nodes: &mut usize,
                    rep_words: bool) -> bool {
    if words_pos.is_empty() {
        return true;
    }
    let mut valid = false;
    let current_word_pos = words_pos.last().unwrap();
    let current_word_board = board.get_word(current_word_pos);

    // get valid words from cache if possible otherwise update cache
    let valid_words = words_map_cache.entry(current_word_board.clone()).or_insert_with(|| {
        get_valid_words(words_len.get(&current_word_pos.len).unwrap_or(&EMPTY_VEC), current_word_board.as_str())
    }).clone();

    // loop thorugh all valid words
    for current_word in valid_words {
        // check if the word has been used
        if !rep_words {
            if words_used.contains(current_word) {
                continue;
            }
        }

        // set word in the board
        board.set_word(current_word_pos, current_word);

        // debug
        *visited_nodes += 1;
        if *visited_nodes % 10_000_000 == 0 {
            board.print();
            println!("Visited nodes: {}M\n", *visited_nodes / 1_000_000);
        }

        // check that exists at least one intersecting word for each letter of the current word
        let mut sol = true;
        for word_pos_intersect in words_intersect.get(current_word_pos).unwrap() {
            let word_board_intersect = board.get_word(word_pos_intersect);
            let words_intersect_num: usize;

            // get valid words from cache if possible otherwise create new vec and update cache
            if let Some(valid_words_cached) = words_map_cache.get(&word_board_intersect) {
                words_intersect_num = valid_words_cached.len();
            }
            else {
                let valid_words_intersect = get_valid_words(words_len.get(&word_pos_intersect.len).unwrap(), word_board_intersect.as_str());
                words_intersect_num = valid_words_intersect.len();

                words_map_cache.insert(word_board_intersect, valid_words_intersect);
            }

            // stop if there are no valid words
            if words_intersect_num == 0 {
                sol = false;
                break;  
            }
        }
        
        // continue recursively if there are intersecting words for each letter of the current word
        if sol {
            if !rep_words {
                words_used.insert(current_word);
            }

            valid = fill_board(board, words_len, &words_pos[..words_pos.len() - 1], words_intersect,
                                words_used, words_map_cache, visited_nodes, rep_words);
            if valid {
                break;
            }
            if !rep_words {
                words_used.remove(current_word);
            }
        }
    }

    if !valid {
        board.set_word(current_word_pos, current_word_board.as_str());
    }

    valid
}


fn fill_board_iter<'a>(board: &mut Board, words_len: &'a HashMap<usize, Vec<&'a str>>, words_pos: &mut Vec<WordPos>,
                    words_intersect: &HashMap<&WordPos, Vec<&WordPos>>, words_used: &mut Vec<&'a str>,
                    words_map_cache: &mut HashMap<String, Vec<&'a str>>, visited_nodes: &mut usize,
                    rep_words: bool) -> bool {
    let mut stack_current_word_pos: Vec<WordPos> = Vec::new();
    stack_current_word_pos.reserve_exact(words_pos.len());

    let mut stack_current_word_board: Vec<String> = Vec::new();
    stack_current_word_board.reserve_exact(words_pos.len());

    let mut stack_valid_words_index: Vec<usize> = Vec::new();

    let mut status = Fs::FWD;     // FWD -> forward; LOOP -> same level; BWD -> backward

    while !(words_pos.is_empty() && status != Fs::BWD) {
        let current_word_pos;
        let current_word_board;
        let current_index;

        // stack
        match status {
            Fs::FWD => {
                current_word_pos = words_pos.pop().unwrap();
                current_word_board = board.get_word(&current_word_pos);
                current_index = 0;
                
                stack_current_word_pos.push(current_word_pos);
                stack_current_word_board.push(current_word_board.clone());
                stack_valid_words_index.push(current_index);
            }
            Fs::LOOP => {
                current_word_pos = *stack_current_word_pos.last().unwrap();
                current_word_board = stack_current_word_board.last().unwrap().clone();
                current_index = *stack_valid_words_index.last().unwrap();
            }
            Fs::BWD => {
                stack_current_word_pos.pop();
                stack_current_word_board.pop();
                stack_valid_words_index.pop();
                words_used.pop();

                current_word_pos = *stack_current_word_pos.last().unwrap();
                current_word_board = stack_current_word_board.last().unwrap().clone();
                current_index = *stack_valid_words_index.last().unwrap();
            }
        }

        // get valid words from cache if possible otherwise update cache
        let valid_words = words_map_cache.entry(current_word_board.clone()).or_insert_with(|| {
            get_valid_words(words_len.get(&current_word_pos.len).unwrap_or(&EMPTY_VEC), current_word_board.as_str())
        }).clone();

        // LOOP REMOVED
        // loop thorugh all valid words
        if current_index >= valid_words.len() {
            status = Fs::BWD;

            // reset board
            words_pos.push(current_word_pos);
            board.set_word(&current_word_pos, current_word_board.as_str());

            continue;
        }

        let current_word = valid_words[current_index];
        let vw_len = stack_valid_words_index.len();
        stack_valid_words_index[vw_len - 1] += 1;

        // check if the word has been used
        if !rep_words {
            if words_used.contains(&current_word) {
                status = Fs::LOOP;
                continue;
            }
        }

        // set word in the board
        board.set_word(&current_word_pos, current_word);

        // debug
        *visited_nodes += 1;
        if *visited_nodes % 10_000_000 == 0 {
            board.print();
            println!("Visited nodes: {}M\n", *visited_nodes / 1_000_000);
        }

        // check that exists at least one intersecting word for each letter of the current word
        let mut sol = true;
        for word_pos_intersect in words_intersect.get(&current_word_pos).unwrap() {
            let word_board_intersect = board.get_word(word_pos_intersect);
            let words_intersect_num: usize;

            // get valid words from cache if possible otherwise create new vec and update cache
            if let Some(valid_words_cached) = words_map_cache.get(&word_board_intersect) {
                words_intersect_num = valid_words_cached.len();
            }
            else {
                let valid_words_intersect = get_valid_words(words_len.get(&word_pos_intersect.len).unwrap(), word_board_intersect.as_str());
                words_intersect_num = valid_words_intersect.len();

                words_map_cache.insert(word_board_intersect, valid_words_intersect);
            }

            // stop if there are no valid words
            if words_intersect_num == 0 {
                sol = false;
                break;  
            }
        }
        
        // continue recursively if there are intersecting words for each letter of the current word
        if sol {
            if !rep_words {
                words_used.push(current_word);
            }

            status = Fs::FWD;
        }
        else {
            status = Fs::LOOP;
        }
    }

    if status != Fs::FWD {
        return false;
    }

    true
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
    println!("\nDEFS");
    for word_pos in words_pos {
        let word = board.get_word(word_pos);
        let defs = json.get(word).unwrap().as_array().unwrap();

        let random_index = rand::thread_rng().gen_range(0..defs.len());
        println!("{:?}: {}", word_pos, defs.get(random_index).unwrap());
    }
}
