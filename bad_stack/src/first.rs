use std::fmt::Debug;

// List a = Empty | Elem a (List a)

#[derive(Debug)]
pub enum List<T> 
where
{
    Empty,
    Elem(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T>
{
    elem: T,
    next: List<T>
}

pub fn cons<T>(data: T, list: List<T>) -> List<T> {

    List::Elem(Box::new(Node{
        elem: data,
        next: list,
    }))
}

pub fn car<T>(list: &List<T>) -> &T {
    
    match list {
        &List::Empty => panic!("Can't call car on empty list"),
        &List::Elem(ref node) => &node.elem,
    }

}

pub fn cdr<T>(list: &List<T>) -> &List<T> {

    match list {
        &List::Empty => panic!("Can't call cdr on empty list"),
        &List::Elem(ref node) => &node.next
    }
}

pub fn map<T, F>(f: F,list: &List<T>) -> List<T> 
where
    F: Fn(&T) -> T,
    T: Clone
{
    match &list {
        &List::Empty => List::Empty,
        &List::Elem(ref node) => List::Elem(Box::new(Node {
            elem: f(&node.elem.clone()), 
            next: map(f, &node.next),
        })), 
    }
}

pub fn filter<T, F>(f: F, list: &List<T>) -> List<T>
where 
    F: Fn(&T) -> bool,
    T: Clone + Debug
{
    match &list {
        &List::Empty => List::Empty,
        &List::Elem(ref node) => {
            if f(&node.elem) {
                List::Elem(Box::new(Node {
                    elem: node.elem.clone(),
                    next: filter(f, &node.next),
                }))
            } else {
                filter(f, &node.next)
            }
        }
    }
}

pub fn list_ref<T>(idx: usize, list: &List<T>) -> &T 
where
    T: Debug
{
    match (idx, list) {
        (0, List::Empty) => panic!("idx is out of range"), 
        (0, List::Elem(ref node)) => &node.elem,
        (_, List::Empty) => panic!("idx is out of range"),
        (idx, List::Elem(ref node)) => list_ref(idx-1, &node.next),
    }
}
