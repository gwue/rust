use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Box<Option<Node<T>>>,
    size: usize,
}

struct Node<T> {
    data: T,
    next: Box<Option<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val:T) -> Self {
        Node {
            data: val,
            next: Box::new(None)
        }
    }

    fn set_next(&mut self, val:T) {
        *self.next = Some(Node::new(val));
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: Box::new(None),
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }

    pub fn push(&mut self, _element: T) {
        match &*self.head {
            None => {
                *self.head = Some(Node::new(_element));
                self.size += 1;
            },
            Some(n) => {
                let mut running_node = n;
                // Find the tail
                loop {
                    match &*running_node.next {
                        Some(next_node) => {
                            running_node = next_node;
                        },
                        None => {
                            running_node.set_next(_element);
                        }
                    }
                }
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
