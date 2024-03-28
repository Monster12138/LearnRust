use std::collections::HashMap;

pub struct MyStruct {}

fn main() {
    let mut m: HashMap<String, Option<()>> = HashMap::new();
    // m.insert("1".to_string(), Some(MyStruct{}));
    m.insert("0".to_string(), None);
}