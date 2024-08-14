fn main () {
    let mut s1:String = String::from("string 1"); // s1 owner of mutable string string 1
    let w1 = &mut s1; // reference of mutable s1 is w1 
    w1.push_str(" add w1");
    println!("w1 = {}",w1);
    let w2 = &mut s1; // reference of mutable s1 is w2, so w1 is gone (because it is mutable i.e. write operation, for non mutable w1 still refers to s1 value)
    w2.push_str(" add w2");
    println!("w2 = {}",w2);
    // println!("w1={}",w1); // error will be thrown, since w1 is invalidated already
    println!("s1 = {}",&mut s1); // same as w2
}