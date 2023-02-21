use std::env;

fn main() {
    println!("こんにちは、世界。");

    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
