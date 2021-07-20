fn main() {
    let mut x = 5;
    println!("The value is {}", x);
    x = 6;
    println!("The value is {}", x);

    let _f: bool = false;

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = _tup;

    let _first = _tup.0;
    let _second = _tup.1;
    let _third = _tup.2;

    let _a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 3, 4, 6, 2];
    let _c = [4; 6];
    
    another_func(4);

    let _y = {
        let x = 5;
        x + 1
    };
}

fn another_func(x: i32) -> i32 {
    println!("The value is {}", x);
    return x;
}
