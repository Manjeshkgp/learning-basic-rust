fn main() {
    let s1: String = String::from("Hello"); // s1 owner
    let (s2, len) = calc_length(s1); // ownership transfer from s1 to arg, and again arg to s2, also length transferred to len
    println!("The main string is {} and it's length is {}", s2, len);
}

fn calc_length(arg: String) -> (String, usize) { // arg will be new owner
    let length: usize = arg.len(); // length is new owner of the string size
    return (arg, length); // again ownership transferred
}
