///List a = Empty | Elem a (List a)
pub enum List {
    Nil,
Cons(i32, Box<List>),
}

///pattern matching on List to get the first element
pub fn first(list: List) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(val, _) => val,
    }
}


fn main () {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{}", first(list));
}

