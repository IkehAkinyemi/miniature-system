use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { elem: elem, next: None, prev: None }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
    
    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            },
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_node| {
            match old_node.borrow_mut().next.take() {
                Some(new_node) => {
                    new_node.borrow_mut().prev.take();
                    self.head = Some(new_node);
                },
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_node).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    
    #[test]
    fn basics() {
        let mut list = List::new();
        
        // Check empty list
        assert_eq!(list.pop_front(), None);
        
        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        
        // Push to check no corruption
        list.push_front(4);
        list.push_front(5);
        
        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        
        // Remove till exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
