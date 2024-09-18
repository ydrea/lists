use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new(self: Self) -> Self { 
        return List { head: Link::Empty };   
    }

    pub fn push(self: &mut Self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(self: &mut Self) -> Option<i32> {
        let result; 
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result= Some(node.elem);
                self.head = node.next;
            }
        };
        result
    }
}