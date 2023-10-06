fn put_str() {
    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('!');
    println!("追加字符 push() -> {}", s);
}

fn insert() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);

    s.insert_str(6, "I like");
    println!("插入字符串 insert_str() -> {}", s);
}

fn replace() {
    let string_replace = String::from("I like rust. Learning rut is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
}

fn replacen() {
    let string_replace = String::from("I like rust. Learning rut is my favorite!");
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
}

fn replace_range() {
    let mut string_replace_range = String::from("I like rust1");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}

fn pop() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn remove() {
    let mut string_remove = String::from("测试 remove 方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    string_remove.remove(0);
    string_remove.remove(3);
    dbg!(string_remove);
}

fn truncate() {
    let mut string_truncate = String::from("测试 truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

fn clear() {
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}

fn add() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello, world!");
}

fn concatenate() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result  += "!!!";

    println!("连接字符串 + -> {}", result);
}

fn format() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

fn escape() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";{}
    println!("What are you doing\x3F (\\x3F means?) {}", byte_escape);

    let unicode_codepoint = "u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string = "String literals
    can span multiple lines.
    The linebreak and indentation here -> \
    <- can be escape too!";
    println!("{}", long_string);
}

fn main() {
    put_str();
    insert();
    replace();
    replacen();
    replace_range();
    pop();
    remove();
    truncate();
    clear();
    concatenate();
    add();
    format();
    escape();
}