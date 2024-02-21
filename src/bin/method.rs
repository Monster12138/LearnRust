struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {width, height}
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}
fn main() {
    let rect1 = Rectangle::new(30, 50);
    println!("{}", rect1.width)
}