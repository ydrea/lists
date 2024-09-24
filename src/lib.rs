pub mod second;
use second::List;

#[cfg(test)]
// mod test {
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);   
        list.push(2);   
        // list.push();   

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        
    }
// }

