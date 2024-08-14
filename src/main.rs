fn main() {
    let x = 5;
    let y = &x; // y is referencing to the value of x
    println!("address of x = {:p}",&x); // getting same addresses
    println!("address of y = {:p}",y); // getting same addresses

    println!("y={}",y); // auto dereferncing = gives value of what y is referring to
}