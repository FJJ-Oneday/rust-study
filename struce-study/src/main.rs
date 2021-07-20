fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("fjj"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        ..user1
    };

    struct Color(i32, i32, i32);
    let _black = Color(0, 0, 0);

    let _user3 = User {
        email: String::from("hello"),
        username: user2.username,
        ..user1
    };

    _user3.username;
    _user3.email;
    _user3.active;
    _user3.sign_in_count;

    build_user(String::from("ddd"), String::from("1233"));

    println!("Hello, world!");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}