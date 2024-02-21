#![allow(unused_variables)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn user() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.active = false;

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user1 active {}", user1.active);
    println!("user2 {:#?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    user();
}