use std::io;
fn main() {
    let mut input = String::new();
    println!("Please enter your name:");
    io::stdin()
        .read_line(&mut input)
        .expect("Expect Failed");
    println!("Use input: {}", input);
}
