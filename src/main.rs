fn main() {
    let s1: String = String::from("Hello");
    let len = calc_length(s1.clone()); 
    println!("The main string is {} and it's length is {}", s1, len);
}

fn calc_length(arg: String) -> usize { 
    return arg.len();
}
