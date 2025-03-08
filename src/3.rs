use std::io;
fn main() {
    let mut user_name = String::new();
    println!("Hello! What is your name?");
    io::stdin().read_line(&mut user_name)
        .expect("Failed to read line");
    println!("Hi, {}!", user_name);
}
