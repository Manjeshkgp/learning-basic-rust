// match

// fn main(){
//     let number = 107;

//     match number {
//         1|3=> println!("Number is One or Three"),
//         2|4=> println!("Number is Two or Four"),
//         5|7=> println!("Number is Five or Seven"),
//         _=>println!("Number is not recognisable")
//     }
// }

fn main() {
    fn is_even(num: isize) -> bool {
        // if num % 2 == 0 {   this also works
        //     return true;
        // } else {
        //     return false;
        // }
        return num % 2 == 0;
    }
    let number = 20;
    match number {
        x if is_even(x) => println!("Even"),
        x if !is_even(x) => println!("Odd"),
        _ => println!("Number not recognizable"),
    }
}
