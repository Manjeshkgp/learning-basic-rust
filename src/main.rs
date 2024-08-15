fn main(){
    let arr:[&str;3] = ["Hello","BY","Manjesh"];
    write_arr(arr); // array directly passed
    println!("arr={:?}",arr); // {:?} is used to format output in a way that is suitable for debugging, if not used then you'll see compilation errors
}

fn write_arr(mut arr1:[&str;3]) { // arr1 new copy of arr, since fixed size arrays and i32, u32, etc. (primitive data types) are stored in stack, not in heap
    arr1[0] = "Fellow";
    println!("arr1={:?}",arr1);
}