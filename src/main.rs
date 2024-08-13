fn main() {
    let s1: String = get_string(); // s1 is the new owner of hello
    println!("s1={}", s1);

    let s2: String = String::from("sssstttrrriiinnnggg 222"); // s2 is owner of the given big string
    let s3: String = send_get_string(s2); // transfer of ownership from s2 to arg, then arg to s3, s3 is new owner of big string
    println!("s3={}", s3);
}

fn get_string() -> String {
    let str = String::from("hello"); //str owner of hello
    return str; // transfer of ownership
}

fn send_get_string(arg: String) -> String { // arg is new owner of the big string
    return arg; // transfer of ownership
}
