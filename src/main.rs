fn main() {
    let x:String = String::from("Hello"); // x is owner of Hello
    process_string(x); // ownership transferred
    // println!("Value of x remains, i.e. {}",x); // this will throw errors, as x is invalid here
}

fn process_string(item:String) { // item is new owner of Hello
    println!("The value of item is {}",item);
}

//after the process_string the `item` becomes invalid, as rust removes it from memory when the scope is ended

// here data types are static, so they work with stack, so no error, but upper example works with heaps and so ownership


// fn main() {
//     let x:u8 = 5;
//     process_integer(x);
//     println!("Value of x remains, i.e. {}",x);
// }

// fn process_integer(item:u8) {
//     println!("The value of item is {}",item);
// }