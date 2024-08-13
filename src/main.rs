fn main() {
    // Create a mutable String s1 with initial content "Hello"
    let mut s1: String = String::from("Hello");
    
    // Borrow s1 as mutable and pass it to the append_string function
    // This allows the function to modify the original String s1
    append_string(&mut s1); 
    
    // After the function call, s1 has been mutated and now contains "Hello extra str"
    println!("The new string s1 becomes {}", s1);
}

// This function takes a mutable reference to a String
// The mutable reference allows the function to modify the String that was passed to it
fn append_string(arg: &mut String) { 
    // Mutate the borrowed String by appending " extra str" to it
    arg.push_str(" extra str");
}
