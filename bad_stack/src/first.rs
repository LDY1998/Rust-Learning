

// List a = Empty | Elem a (List a)

#[derive(Debug)]
pub enum List<T> {
    Empty,
    Elem(T, Box<List<T>>),
}

impl<T> List<T> {

    pub fn insert(position: i8, data: T) {


    }
}

