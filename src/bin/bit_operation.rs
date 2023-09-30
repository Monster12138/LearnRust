fn bit_operator() {
    let a: i32 = 2;
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn main() {
    bit_operator();
}
