///got took
///List = Link Option(BoxNode) =+ Link Option(BoxNode) =+ None
//init type List, first pointer...
pub struct List<T> {
 pub a: Link<T>,
}
//...to Option...
pub type Link<T> = Option<Box<Node<T>>>;

//...shaped like a recursive Node, on the heap (Box).
pub struct Node<T> {
    car: T,
    cdr: Link<T>
}
///Method - create an empty List
impl<T> List<T> {
    pub fn new() -> Self { 
        List { a: None }   
    }
//Associated fn - push a node to List
    pub fn push(&mut self, car: T) {
 //make a new_node to be Box new(elem, next: self.a)
        let new_node = Box::new(Node {
            car,
            //temporarily replacing self.a with an empty Link 
            cdr: self.a.take(),
        });
        //< returning self.a to be the new_node
        self.a = Some(new_node);
    }

//match a peek
    pub fn peek(&self)->Option<&T>{
        match &self.a {
            None => None,
            Some(val) => Some(&val.car)
        }
    }

    // Peek at the value in the head node (if it exists)
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.a.as_mut() {
            None => None,               // If the list is empty, return None
            Some(val) => Some(&mut val.car), // If there is a node, return a mutable reference to `node.car`
        }
    }
/// Pops a node off the list and returns its value (if the list is not empty).
///
/// This method temporarily takes ownership of the head node (`self.a`) by using `take()`,
/// which replaces the head of the list with `None`. Then, it uses `map` to either:
/// - If the list is empty (`None`), it returns `None`.
/// - If the list has a node (`Some`), it moves to the next node by assigning `self.a` to the `cdr` (the next node),
///   and returns the value (`car`) stored in the popped node.
///
/// # Returns
/// - `Some(T)` if the list had at least one node, with `T` being the value from the popped node.
/// - `None` if the list was empty.
///
/// # Example
/// ```
/// let mut list = List::new();
/// list.push(1);
/// list.push(2);
/// assert_eq!(list.pop(), Some(2));  // Pops the most recent element
/// assert_eq!(list.pop(), Some(1));  // Pops the next element
/// assert_eq!(list.pop(), None);     // List is now empty
/// ```
pub fn pop(&mut self) -> Option<T> {
    self.a.take() // Temporarily replace self.a with None
        .map(|node| {
            // If node exists, move to the next node (node.cdr) and return node.car
            self.a = node.cdr;
            node.car
        })
}
// //Option SomeT or None, to check for empty List
//     pub fn pop(&mut self) -> Option<T> {
//         //init result to be returned
//         let result; 
//         //mach on Option
//         //+ replace, temporarily replacing self.a with None
//         match self.a.take() {
//             None => {
//                 result = None;
//             },
//             Some(b) => {
//                 result = Some(b.car);
//                 self.a = b.cdr;
//             },
//         };
//         result
//     }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut drop_link = self.a.take();
        // while let == do this thing until this pattern stops matching
            // boxed_node goes out of scope and gets dropped here...
        while let Some(mut boxed_node) = drop_link {

//  ...while Node next field = Link Empty.
            drop_link = boxed_node.cdr.take();
        } //so no unbounded recursion occurs
    }
}
