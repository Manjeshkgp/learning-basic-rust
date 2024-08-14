fn main() {
    let mut x = 5;
    x = x+1; // don't use dereferncing operator here, not worj
    let y = &mut x; // y is referering to the value of mut x, and having address of mut x as it's value
    // y=y+1; can't do this, as this will work with the address of x, which throws error
    *y=*y+1; // that's why we used dereferencing operator
    println!("x={}",y);
}