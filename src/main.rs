use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

fn main() {
    println!("Crosswords Generator v0.1");

    // Load json words and definitions
    let time_json = SystemTime::now();
    let json = load_words("./data/words.txt");
    println!("Time to read and parse json: {} ms", time_json.elapsed().unwrap().as_millis());
    // println!("Words Text: {}", json.get("AA").unwrap());

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

    // next
    
}

fn load_words(path: &str) -> serde_json::Value {
    let content = fs::read_to_string(path).expect("Unable to read text file");
    let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
    json
}

/*
fn load_words(path: &str) -> Result<serde_json::Value, std::io::Error> {
    let json = match fs::read_to_string(path) {
        Ok(content) => {
            let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
            json
        },
        Err(e) => {
            return Err(e)
        },
    };
    Ok(json)
}
*/
