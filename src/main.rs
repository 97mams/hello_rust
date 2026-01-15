use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() > 2 {
    //     println!("Usage: cargo run -- <name>");
    //     return;
    // } 
    let name = &args[1];
    let age = &args[2];
    println!("Hello, {}! You are {} years old.", name, age);
}
