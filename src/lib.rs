pub mod second;
pub mod first;

/// Adds two
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test {
    use super::*;
use std::fmt::Debug;
    use crate::second::{List, Link};
    

    #[test]
    fn peekt(){
        // let mut list = List::new();
        // assert_eq!(list.pop(), None);

    //     list.push(1);   
    //     list.push(2);   
    //     // list.push();   

    //     assert_eq!(list.pop(), Some(2));
    //     assert_eq!(list.pop(), Some(1));
    //     assert_eq!(list.pop(), None);
        }


    #[test]
    #[derive(Debug, PartialEq)]
    fn tester<T> ()
    where T: Debug + PartialEq
    {
        let mut list: second::List<T> = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);   
        list.push(2);   
        // list.push();   

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
    }
}
