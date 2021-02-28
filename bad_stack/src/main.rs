use first::List;

mod first;

fn main() {
   
    let list: List<i32> = List::Elem(1, Box::new(List::Empty));

    println!("{:?}", list);
    
}
