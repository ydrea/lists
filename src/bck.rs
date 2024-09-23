pub struct List<T> {
    pub a: Link<T>,  // Pointer to the head of the list
}

pub type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    car: T,        // The value stored in the node
    cdr: Link<T>,  // The next node in the list
}

impl<T> List<T> {
    // Create a new empty List
    pub fn new() -> Self {
        List { a: None }
    }

    // Push a new element to the front of the list
    pub fn push(&mut self, car: T) {
        let new_node = Box::new(Node {
            car,                       // Store the value in the new node
            cdr: self.a.take(),       // Take the current head and assign it as the next node
        });
        self.a = Some(new_node);      // The new node becomes the head of the list
    }

    // Peek at the value in the head node (if it exists)
    pub fn peek(&self) -> Option<&T> {
        match &self.a {
            None => None,               // If the list is empty, return None
            Some(val) => Some(&val.car), // Return a reference to the value (car)
        }
    }
}
