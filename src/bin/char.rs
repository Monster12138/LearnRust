fn main() {
    let a = 'a';
    println!("字符'{}'占用了{}个字节", a, std::mem::size_of_val(&a));
}