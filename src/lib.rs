pub mod lists {

    pub struct Stack<T> {
        head: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    struct Node<T> {
        item: T,
        next: Link<T>,
    }

    impl<T> Node<T> {
        fn with(item: T) -> Self {
            Self { item: item, next: None }
        }

    }

    pub struct IntoIter<T>(Stack<T>);
    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }
    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }

    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_mut().map(|node| &mut **node);
                &mut node.item
            })
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {

        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_ref().map(|node| &**node);
                &node.item
            })
        }


    }

    impl<'a, T> Stack<T> {
        pub fn new() -> Self {
            Self { head: None }
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut{ next: self.head.as_mut().map(|node| &mut ** node) }
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter { next: self.head.as_ref().map(|node| &**node) }
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        pub fn push(&mut self, item: T) {
            let new_node = Box::new(Node {
                item,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|boxed| &boxed.item)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|boxed| &mut boxed.item)
        }
        
        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                // automatically Derefs the Box when calling node.item
                // and then wrapts the item in Option as per 
                // the rules of Option<T>::map
                node.item
            })
        }
    }

    impl<T> Drop for Stack<T> {
        
        // When dropping the stack,
        // we drop (5 lines under) all the boxes (get the Option<...>
        // then let it get dropped, replacing it with None at the same time)
        fn drop(&mut self) {
            let mut curr_link = self.head.take();
            while let Some(mut box_node) = curr_link {
                curr_link = box_node.next.take();
            }

        }

    }

}


#[cfg(test)]
mod tests {
    use super::lists::Stack;

    #[test]
    fn basic(){
        let mut stack :Stack<u32> = Stack::new(); 
        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);

        for node in stack.iter() { 
            println!("{:?}", node);
        }
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1); stack.push(2); stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        stack.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }
}
