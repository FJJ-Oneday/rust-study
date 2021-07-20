#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width: 8, height: 7};
        let smaller = Rectangle{width: 5, height: 1};

        assert!(larger.can_hold(&smaller));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}