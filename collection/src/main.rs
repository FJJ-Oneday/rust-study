use std::collections::HashMap;

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    vec.push(1);
    vec.push(2);
    vec.push(4);
    vec.push(5);

    let _third: &i32 = &v[2];

    for i in &v {
        print!("{}, ", i);
    }

    let _row = vec![
        SpreadsheetCell::Float(12.1),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
    ];

    let s1 = "hello".to_string();
    let s2 = ", world.".to_string();
    let s3 = format!("{}{}", s1, s2);
    println!("{}", s3);

    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("yello", 50);

    let teams = vec!["blue", "yello"];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for (key, value) in &scores {
        println!("team_name: {}, score: {}", key, value);
    }

    scores.entry(&"red").or_insert(&20);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
