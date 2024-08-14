// Dangling Referrence

fn main() {
    let a = String::from("hello");
    let refer_to_nothing = give_me_reffer(a);
}

fn give_me_reffer()->&String {
    let s = String::from("hello");
    return &s; // scope of s is ended here that's why refer_to_nothing will be invalid, so this will throw error
}