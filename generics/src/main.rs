fn main() {
    let number_list = vec![34, 56, 90, 12, 9];
    let result = largest(&number_list);
    println!("largest number is {}", result);

    let char_list = vec!['y', 'a', 'b', 'x', 'w'];
    let result = largest(&char_list);
    println!("largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn notify(itme: impl Summary) {
    println!("{}", itme.summarize());
}
