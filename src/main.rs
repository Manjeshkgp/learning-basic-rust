fn main() {
    // let mut num: u8 = 5;
    // println!("This is store in num {}", num);
    // num = 190;
    // println!("This is store in num {}", num);

    // let mut sentence: String = String::from("Some characters");
    // sentence.push_str(" some additions");
    // print!("{}",sentence);

    // tupple
    // let emp_info:(&str,u8) = ("Ramesh",50);
    
    // let emp_name: &str = emp_info.0;
    // let em_age: u8 = emp_info.1;

    // desctructuring

    // let (employee_name,employee_age) = emp_info;

    // print!("Employee name is {} and age is {}",employee_name,employee_age);
    let num_1 = 5;
    let num_2 = 10;
    let result:u8 = add(num_1,num_2);
    print!("The sum of num_1 and num_2 is: {}",result);
}
 

// functions

fn add(item_1:u8,item_2:u8)->u8 {
    return  item_1+item_2;
}