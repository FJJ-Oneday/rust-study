use std::cell::RefCell;

fn main() {
    let stack = RefCell::new(Vec::new());
    
    stack.borrow_mut().push(1);
    stack.borrow_mut().push(2);
    stack.borrow_mut().push(3);

    while let Some(top) = stack.borrow_mut().pop() {
        println!("{}", top);
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1 ..= 5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a' ..= 'j' => println!("early ASSCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point {x: 0, y: 1};
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(1, y);

    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello {id: 5};

    match msg {
        Message::Hello {id: id_variable @ 3 ..= 7} => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello {id: 10..= 12} => println!("Found an id in another range."),
        Message::Hello {id} => println!("Found some other id: {}", id),
    }
}