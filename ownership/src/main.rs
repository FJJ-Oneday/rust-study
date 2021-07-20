fn main() {
    let s = String::from("hello");

    // s.push_str(", world");

    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1={}, s2={}", s1, s2);

    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);
    let s3_length = calculate_length(&s3);

    first_word(&s3[..]);

    println!("s1={}, s3={}, s3_length={}", s1, s3, s3_length);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}