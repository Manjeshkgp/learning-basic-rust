// Vector - Dynamic Array

fn main() {
    // let mut v:Vec<i32> = Vec::new(); // declaration
    // // let mut v = Vec::<i32>::new(); // declaration also can be done via this
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // println!("v={:?}",v);
    let mut v = vec![1,2,3,4,5];
    v.pop();
    v.push(6);
    println!("v={:?}",v);
}