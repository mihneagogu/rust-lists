mod lists {

    pub struct Stack<T> {
        head: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;
    struct Node<T> {
        item: T,
        next: Link<T>,
    }

    impl<T> Node<T> {
        fn with(item: T) -> Self {
            Self { item: item, next: None }
        }

    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self { head: None }
        }

        pub fn push(&mut self, item: T) {
            let new_node = Box::new(Node {
                item,
                next: self.head.take(),
            });
            self.head = Some(new_node);
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
    }

}
