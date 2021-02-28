

// List a = Empty | Elem a (List a)

#[derive(Debug)]
pub enum List<T> {
    Empty,
    Elem(T, Box<List<T>>),
}

pub fn cons<T>(data: T, list: List<T>) -> List<T> {

    List::Elem(data, Box::new(list))
}

pub fn car<T>(list: &List<T>) -> &T {
    
    match list {
        &List::Empty => panic!("Can't call car on empty list"),
        &List::Elem(ref data, _) => &data,
    }

}

pub fn cdr<T>(list: &List<T>) -> &List<T> {

    match list {
        &List::Empty => panic!("Can't call cdr on empty list"),
        &List::Elem(_, ref next) => &next
    }
}

pub fn map<T, F>(f: F,list: &List<T>) -> List<T> 
where
    F: Fn(&T) -> T
{
    match &list {
        &List::Empty => List::Empty,
        &List::Elem(ref data, next) => List::Elem(f(data), Box::new(map(f, &*next))), 
    }
}

pub fn filter<T, F>(f: F, list: &List<T>) -> List<T>
where 
    F: Fn(&T) -> bool,
    T: Clone,
{
    match &list {
        &List::Empty => List::Empty,
        &List::Elem(data, next) => {
            if f(&data) {
                List::Elem(data.clone(), Box::new(filter(f, &*next)))
            } else {
                filter(f, &*next)
            }
        }
    }
}

pub fn list_ref<T>(idx: usize, list: &List<T>) -> &T {
    match (idx, list) {
        (0, List::Empty) => panic!("idx is out of range"), 
        (0, List::Elem(ref data, _)) => data,
        (_, List::Empty) => panic!("idx is out of range"),
        (idx, List::Elem(_, next)) => list_ref(idx-1, &*next),
    }
}
