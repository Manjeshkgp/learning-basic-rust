fn main() {
    let s1: String = String::from("Hello");
    let len = calc_length(&s1); // passing reference of s1 || borrow operation
    println!("The main string is {} and it's length is {}", s1, len);
}

fn calc_length(arg: &String) -> usize { // arg refers to value of s1 but does not own it
    return arg.len(); // after borrowing, no write operations can be done, only read operations allowed
}
