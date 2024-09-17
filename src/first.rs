///List a = Empty | Elem a (List a)
pub enum List {
    Nil,
Cons(i32, Box<List>),
}

pub fn first(list: List) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(val, _) => val,
    }
}