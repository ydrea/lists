///got took
///List = Link Option(BoxNode) =+ Link Option(BoxNode) =+ None
//init type List, first pointer...
pub struct List {
 pub a: Link,
}
//...to Option...
pub type Link = Option<Box<Node>>;

//...shaped like a recursive Node, on the heap (Box).
pub struct Node {
    car: i32,
    cdr: Link
}
///create an empty List
impl List {
    pub fn new(&self) -> Self { 
        return List { a: None };   
    }
///push a node to List
    pub fn push(&mut self, car: i32) {
 //make a new_node to be Box new(elem, next: self.a)
        let new_node = Box::new(Node {
            car,
            //temporarily replacing self.a with an empty Link 
            cdr: self.a.take(),
        });
        //< returning self.a to be the new_node
        self.a = Some(new_node);
    }
///pop a Node off the List
///Option SomeT or None, to check for empty List
    pub fn pop(&mut self) -> Option<i32> {
        //init result to be returned
        let result; 
        //mach on Option
        //+ replace, temporarily replacing self.a with None
        match self.a.take() {
            None => {
                result = None;
            },
            Some(node) => {
                result = Some(node.car);
                self.a = node.cdr;
            },
        };
        result
    }
}
impl Drop for List {
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
