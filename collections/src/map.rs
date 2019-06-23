use std::collections::HashMap;

pub fn map_test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // zip example
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{} - {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(80);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(20);

    for (key, value) in &scores {
        println!("{} - {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut maps = HashMap::new();

    for word in text.split_whitespace() {
        let count = maps.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", maps);
}