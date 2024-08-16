// loops

// fn main() {
//     let mut i = 0;
//     while i<=5 {
//         println!("{}",i);
//         i+=1;
//     }
// }

// fn main() {
//     loop {
//         println!("Infinite Printing");
//     }
// }

fn main () {
    let arr = [1,2,3];
    for element in &arr{
        println!("element = {}",element);
    }
}