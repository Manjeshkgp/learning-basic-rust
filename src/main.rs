//Shadowing
fn main() {
    let x = 5; // all of the x are declared, not mutated, so it's allowed
    println!("x1={}",x);
    let x = "Hello"; // all of the x are declared, not mutated
    println!("x2={}",x);
    let x = x.len(); // all of the x are declared, not mutated, so it's allowed
    println!("x3={}",x);

    // x = 9; // this is not allowed
}