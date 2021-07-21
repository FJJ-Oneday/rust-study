use oop::{Draw, Screen, Button};
use oop::Post;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("yes"),]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectBox");
    }
}