
async fn do_something() {
    println!("go go go!");
}

async fn hello_world() -> bool {
    hello_cat().await;
    println!("hello world");
    true
}

async fn hello_cat() {
    println!("hello cat");
}


fn main() {

}