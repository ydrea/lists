pub mod second;
pub mod first;

/// Adds two
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::first::{List, Link};
    use crate::second::{List, Link};
    

    // #[test]
    // fn testis (){
    //     let mut list = List::new(&first::List{a: Link::Empty});
    //     assert_eq!(list.pop(), None);

    //     list.push(1);   
    //     list.push(2);   
    //     // list.push();   

    //     assert_eq!(list.pop(), Some(2));
    //     assert_eq!(list.pop(), Some(1));
    //     assert_eq!(list.pop(), None);
    //     }


    #[test]
    fn tester (){
        let mut list = List::new(&second::List{a: None});
        assert_eq!(list.pop(), None);

        list.push(1);   
        list.push(2);   
        // list.push();   

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
    }
}
