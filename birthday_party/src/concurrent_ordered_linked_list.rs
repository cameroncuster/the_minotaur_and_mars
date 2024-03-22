use std::sync::Mutex;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Ord + Clone,
{
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

pub struct ConcurrentOrderedLinkedList<T> {
    head: Mutex<Option<Box<Node<T>>>>,
}

impl<T> ConcurrentOrderedLinkedList<T>
where
    T: Ord + Clone + Copy,
{
    pub fn new() -> Self {
        ConcurrentOrderedLinkedList {
            head: Mutex::new(None),
        }
    }

    pub fn insert(&self, data: T) {
        let mut head = self.head.lock().unwrap();
        let mut current = head.as_mut();

        if current.is_none() {
            *head = Some(Box::new(Node::new(data, None)));
            return;
        }

        if current.as_ref().unwrap().data > data {
            let new_node = Box::new(Node::new(data, head.take()));
            *head = Some(new_node);
            return;
        }

        while let Some(node) = current {
            if let Some(next) = node.next.as_mut() {
                if next.data > data {
                    node.next = Some(Box::new(Node::new(data, node.next.take())));
                    return;
                }
            } else {
                node.next = Some(Box::new(Node::new(data, None)));
                return;
            }
            current = node.next.as_mut();
        }

        unreachable!()
    }

    pub fn remove(&self, data: T) -> bool {
        let mut head = self.head.lock().unwrap();
        let mut current = head.as_mut();

        if current.is_none() {
            return false;
        }

        while let Some(node) = current {
            if let Some(next) = node.next.as_mut() {
                if next.data == data {
                    node.next = next.next.take();
                    return true;
                }
            }

            current = node.next.as_mut();
        }

        false
    }

    pub fn pop(&self) -> Option<T> {
        let mut head = self.head.lock().unwrap();
        let current = head.take();

        if current.is_none() {
            return None;
        }

        let mut node = current.unwrap();
        *head = node.next.take();
        Some(node.data)
    }

    pub fn contains(&self, data: T) -> bool {
        let mut head = self.head.lock().unwrap();
        let mut current = head.as_mut();

        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = node.next.as_mut();
        }

        false
    }

    pub fn is_empty(&self) -> bool {
        let head = self.head.lock().unwrap();
        head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let list = ConcurrentOrderedLinkedList::new();
        list.insert(1);
        list.insert(3);
        list.insert(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_remove() {
        let list = ConcurrentOrderedLinkedList::new();
        list.insert(1);
        list.insert(3);
        list.insert(2);

        list.remove(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_pop() {
        let list = ConcurrentOrderedLinkedList::new();
        list.insert(1);
        list.insert(3);
        list.insert(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_contains() {
        let list = ConcurrentOrderedLinkedList::new();
        list.insert(1);
        list.insert(3);
        list.insert(2);

        assert_eq!(list.contains(2), true);
        assert_eq!(list.contains(4), false);
    }

    #[test]
    fn test_is_empty() {
        let list = ConcurrentOrderedLinkedList::new();
        assert_eq!(list.is_empty(), true);

        list.insert(1);
        assert_eq!(list.is_empty(), false);
    }
}
