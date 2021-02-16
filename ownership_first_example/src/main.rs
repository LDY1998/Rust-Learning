
use std::string::String;
use std::collections::HashMap;

mod employee;
use employee::employee_manage;

fn main() {

    let mut s2 = String::from("hello");

    let word = first_word(&s2);

    println!("Word is {}", word);


    s2.clear();

    let mut employee_manage = employee_manage::new();
    employee_manage.add_employee(String::from("apple"), String::from("computer science"));

    employee_manage.list_all();


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.area() >= other.area();
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle{
            width: size,
            height: size,
        }
    }


}