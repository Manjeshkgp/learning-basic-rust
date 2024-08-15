fn main(){
    let mut arr:[&str;3] = ["Hello","BY","Manjesh"];
    write_arr(&mut arr); // reference of mut arr is passed
    println!("arr={:?}",arr); 
}

fn write_arr(arr1:&mut [&str;3]){
    arr1[0] = "Updated arr[0]";
    println!("arr1={:?}",arr1);
}

// both arr & arr1 will be same, as arr1 edited the main mut arr