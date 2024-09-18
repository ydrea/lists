// pub mod first;
pub mod firstdotone;

/// Adds two
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::firstdotone::{List, Link};
    
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn testis (){
        let mut list = List::new(&firstdotone::List{a: Link::Empty});
        assert_eq!(list.pop(), None);

        list.push(1);   
        list.push(2);   
        // list.push();   

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
    }
    
    



}
