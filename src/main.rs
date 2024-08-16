// fn main() {
//     let vrr: Vec<&str>  = vec!["Hello","test"]; // vrr is owner
//     write_vrr(vrr); // ownership transferred
//     println!("vrr={:?}",vrr); // this definitely throws error
// }

// fn write_vrr(vrr2:Vec<&str>){ // vrr2 new owner
//     println!("vrr2={:?}",vrr2);
// }

fn main() {
    let mut vrr: Vec<&str>  = vec!["Hello","test"]; // vrr is owner
    write_vrr(&mut vrr); // vrr2 refers to the value of mut vrr
    println!("vrr={:?}",vrr);
}

fn write_vrr(vrr2:&mut Vec<&str>){ // borrowing happened
    vrr2.push("Added str");
}