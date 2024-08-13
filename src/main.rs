fn main() {
    let str1 = String::from("Hello"); // str1 is owner of hello
    let str2 = str1; // transfer of ownership, str2 is new owner, str1 becomes invalidated
    // println!("str1 = {}",str1); // this will throw error, as str2 is the owner and str1 is invalid
    println!("str2 = {}",str2);
}