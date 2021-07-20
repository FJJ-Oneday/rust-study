fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!("the area of the rectangle is {} square pixels.", area(&rect1));
    println!("the area of the rectangle is {} square pixels.", rect1.area());
    let rect2 = Rectangle::square(3);
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("{:#?}", rect1);
    
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
