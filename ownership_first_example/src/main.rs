




fn main() {


    // type: &str, local variable in stack, cannot mutate
    let s = "1234"; 

    // type: std::String, allocate in heap, can be mutated
    let mut s = String::from("1234");

    s.push_str("5");

    println!("{}",s);

}

