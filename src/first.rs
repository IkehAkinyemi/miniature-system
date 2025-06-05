use std::mem;

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    
    pub fn push(&mut self, elem: i32) {
        let node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        
        self.head = Link::More(Box::new(node))
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None, 
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    
    #[test]
    fn basics() {
        let mut list = List::new();
        
        // check empty list behaves correctly.
        assert_eq!(list.pop(), None);
        
        // populate list
        list.push(1);
        list.push(2);
        list.push(3);
        
        // check removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        
        // add more nodes to ensure nothing was corrupted
        list.push(4);
        list.push(5);
        
        // check removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        
        // check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
