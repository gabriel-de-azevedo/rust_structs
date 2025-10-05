struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.active);
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.sign_in_count);
}
