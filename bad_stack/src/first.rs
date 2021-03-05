use std::fmt::Debug;
use std::mem;

// List a = Empty | Elem a (List a)

#[derive(Debug)]
pub enum List<T> 
where T: Copy
{
    Empty,
    Elem(Box<Node<T>>),
}


#[derive(Debug)]
pub struct Node<T>
where T: Copy
{
    elem: T,
    next: List<T>
}

pub fn cons<T>(data: T, list: List<T>) -> List<T> 
where T: Copy
{

    List::Elem(Box::new(Node{
        elem: data,
        next: list,
    }))
}

pub fn car<T>(list: &List<T>) -> &T 
where T: Copy
{
    
    match list {
        &List::Empty => panic!("Can't call car on empty list"),
        &List::Elem(ref node) => &node.elem,
    }

}

pub fn cdr<T>(list: &List<T>) -> &List<T> 
where T: Copy
{

    match list {
        &List::Empty => panic!("Can't call cdr on empty list"),
        &List::Elem(ref node) => &node.next
    }
}

pub fn map<T, F>(f: F,list: &List<T>) -> List<T> 
where
    F: Fn(&T) -> T,
    T: Copy
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
    T: Copy + Debug
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
    T: Debug + Copy
{
    match (idx, list) {
        (0, List::Empty) => panic!("idx is out of range"), 
        (0, List::Elem(ref node)) => &node.elem,
        (_, List::Empty) => panic!("idx is out of range"),
        (idx, List::Elem(ref node)) => list_ref(idx-1, &node.next),
    }
}

#[derive(Debug)]
pub struct Stack<T> 
where T: Copy
{
    head: List<T>
}


impl<T> Stack<T> 
where T: Copy
{
    pub fn new() -> Stack<T> {
        Stack {
            head: List::Empty,
        }
    }

    pub fn push(&mut self, data: T) {
        
        // self.head = Box::new(cons(data, self.head));
        
        let new_node = Node {
            elem: data,
            next: mem::replace(&mut self.head, List::Empty),
        };

        self.head = List::Elem(Box::new(new_node));
        
    }

    pub fn pop(&mut self) -> Option<T> {

        match mem::replace(&mut self.head, List::Empty) {
            List::Empty => None,
            List::Elem(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }


    }
}

mod test {
    use super::Stack;
    #[test]
    fn basic() {
        let mut st = Stack::<i32>::new();

        for i in 1..10 {
            st.push(i);
        }

        for i in 9..0 {
            assert_eq!(st.pop().unwrap(), i);
        }
    }
}