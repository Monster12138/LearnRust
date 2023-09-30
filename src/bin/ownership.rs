fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string
}

fn main() {
    let mut s = String::from("hello");
    takes_ownership(s);

    s = gives_ownership();
    println!("{}", s);
}
