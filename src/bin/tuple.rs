#![allow(unused_variables)]
fn get_variable() {
    let tup = (500, 6.4, 1);
    let (x, y ,z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The values: {} {} {}", five_hundred, six_point_four, one);
}


fn main() {
    get_variable();

}