use std::fs;

fn main() {
    println!("Crosswords Generator v0.1");

    // test
    let asd = test("asd");
    let pog = asd.get(0);
    println!("Vec[0]: {}", pog.unwrap());

    // load words
    let json = load_words("./data/words.txt");
    println!("Words Text: {}", json.get("AA").unwrap());
}

fn test(path: &str) -> Vec<&str> {
    println!("Words Path: {path}");
    let mut a = Vec::<&str>::new();
    a.push("pog");
    a
}

fn load_words(path: &str) -> serde_json::Value {
    let content = fs::read_to_string(path).unwrap();
    let json: serde_json::Value = serde_json::from_str(content.as_str()).expect("JSON was not well-formatted");
    json
}
