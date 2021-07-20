fn main() {
    // let number = 6;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
    
    // println!("{}", number);
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else {
    //     println!("number is not divisible by 4, 3");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let a = [1, 2, 3, 4];

    for element in a.iter() {
        println!("{}", element);
    }

    for i in (1..4).rev() {
        println!("{}!", i);
    }
}
