struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        active: false,
        username: String::from("another@example.com"),
        email: String::from("another@example.com"),
        sign_in_count: 21,
    };

    let user2 = User {
        active: true,
        email: String::from("rdlnk@example.com"),
        ..user1
    };

    let color1 = Color(0, 32, 0);

    println!("{}", user2.username);
}
