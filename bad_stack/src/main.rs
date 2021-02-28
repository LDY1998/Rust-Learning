use first::{List, cons, car, cdr, map, filter, list_ref};
use std::cmp::Ordering;

mod first;

fn main() {


    let li2: List<i32> = cons(1, cons(2, List::Empty));
    println!("list2: {:?}", li2);

    println!("Head of li2: {}", car(&li2));
    println!("Second of li2: {:?}", cdr(&li2));

    let li3 = map(|x| {
        x+1
    }, &li2);

    println!("li2 after map: {:?}", li3);

    println!("original li2: {:?}", li2);

    let filtered_li2 = filter(|x| {
        match x.cmp(&1) {
            Ordering::Equal => true,
            _ => false,
        }
    }, &li2);

    println!("List after filtered: {:?}", filtered_li2);

    println!("original li2: {:?}", li2);

    let first = list_ref(0, &li2);
    let second = list_ref(1, &li2);

    println!("first: {}, second: {}", first, second);

    println!("third: {}", list_ref(2, &li2));
}