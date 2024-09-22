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
    pub fn new(&self) -> Self { 
        return List { a: None };   
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
///pop a Node off the List w map&closure
pub fn pop(&mut self)->Option<T>{
    self.a.take()//temporarily replacing self.a with None
    
    .map(|node|{

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
