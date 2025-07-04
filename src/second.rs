pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }
    
    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        
        self.head = Link::Some(node);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.elem
        })
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
      IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
      Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node) }
      // Iter { next: self.head.as_deref() }
    }
    
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}


pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
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
    
    #[test]
    fn peek() {
        let mut list = List::new();
        
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        list.peek_mut().map(|value| {
          *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn into_iter() {
      let mut list = List::new();
      list.push(1); list.push(2); list.push(3);

      let mut list_iter = list.into_iter();
      assert_eq!(list_iter.next(), Some(3));
      assert_eq!(list_iter.next(), Some(2));
      assert_eq!(list_iter.next(), Some(1));
      assert_eq!(list_iter.next(), None);
    }
    
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None)
    }
    
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        
        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 1));
    }
}
