fn main() {
    let s1:String = String::from("Hello"); // s1 owner
    let s1_len:usize = calc_length(s1); // ownership transfered
    println!("s1={} and it's length is {}",s1,s1_len); // this will throw error
}

fn calc_length (arg:String)->usize { // arg will be new owner
    return arg.len(); // returned 5
}