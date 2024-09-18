use std::mem;
///List = Link::More(Box<Node>) =+ Link::More(Box<Node>) =+ Link::Empty
//init type List, first pointer...
pub struct List {
 pub a: Link,
}
///...to either nothing, or something more...
pub enum Link {
    Empty,
    More(Box<Node>),
}
///...shaped like a Node, on the heap (Box).
pub struct Node {
    elem: i32,
    next: Link
}
///create an empty List
impl List {
    pub fn new(&self) -> Self { 
        return List { a: Link::Empty };   
    }
///push a node to List
    pub fn push(&mut self, elem: i32) {
 //make a new_node to be Box::new(elem, next: self.a)
        let new_node = Box::new(Node {
            elem: elem,
            //temporarily replacing self.a with an empty Link 
            next: mem::replace(&mut self.a, Link::Empty),
        });
        //< returning self.a to be the new_node
        self.a = Link::More(new_node);
    }
///pop a Node off the List
///Option::Some<T> || None, to check for empty List
    pub fn pop(&mut self) -> Option<i32> {
        //init result to be returned
        let result; 
        //mach on Option
        //+ replace, temporarily replacing self.a with an empty Link 
        match mem::replace(&mut self.a, Link::Empty) {
            Link::Empty => {
                result = None;
            },
            Link::More(node) => {
                result = Some(node.elem);
                self.a = node.next;
            },
        };
        result
    }
}
impl Drop for List {
    fn drop(&mut self) {
        let mut drop_link = mem::replace(&mut self.a, Link::Empty);
        // while let == do this thing until this pattern stops matching
            // boxed_node goes out of scope and gets dropped here...
        while let Link::More(mut boxed_node) = drop_link {

//  ...while Node::next field = Link::Empty.
            drop_link = mem::replace(&mut boxed_node.next, Link::Empty);
        } //so no unbounded recursion occurs
    }
}
