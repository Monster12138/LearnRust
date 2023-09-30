fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multi_borrowing() {
    let mut s = String::from("hello");
    let b1 = &s;
    let b2 = &s;

    println!("{} and {}", *b1, *b2);

    let b3 = &mut s;
    println!("{}", b3);
}

fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("{}", s1);

    multi_borrowing();
}