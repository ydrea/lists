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
        return List { a: None };   
    }

    //Associated fn - push a node to List
    pub fn push(&mut self, car: T) {
 //make a new_node to be Box new(car, cdr: self.a)
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


    pub fn pop(&mut self) -> Option<T> {
        self.a.take() // Temporarily replace self.a with None
            .map(|node| {
                // If node exists, move to the next node (node.cdr) and return node.car
                self.a = node.cdr;
                node.car
            })
    }
    
    // Peek at the value in the head node (if it exists)
    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    //     match self.a.as_mut() {
    //         None => None,               // If the list is empty, return None
    //         Some(val) => Some(&mut val.car), // If there is a node, return a mutable reference to `node.car`
    //     }
    // }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.a.as_mut().map(|node| {
            &mut node.car
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { cdr: self.a.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { cdr: self.a.as_deref_mut() }
    }
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

// IntoIter
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// Iter
pub struct Iter<'a, T> {
    cdr: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cdr.map(|node| {
            self.cdr = node.cdr.as_deref();
            &node.car
        })
    }
}

//IterMut
pub struct IterMut<'a, T> {
    cdr: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cdr.take().map(|node| {
            self.cdr = node.cdr.as_deref_mut();
            &mut node.car
        })
    }
}


#[cfg(test)]
mod test {
use super::List;
#[test]
    fn basics() {
        let list = &mut List::new();
                assert_eq!(list.pop(), None);
        list.push(1);   
        list.push(2);   
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
    }
}

