use std::iter::FromIterator;
use std::fmt::{Debug};

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize
}

impl<T> SimpleLinkedList<T> where T: Debug + Copy {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node {
            data: element,
            next: self.head.take()
        });
        self.len += 1;
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let node = self.head.take().unwrap();
        self.head = node.next;
        self.len -= 1;

        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {
            return None;
        }

        Some(&self.head.as_ref().unwrap().data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let vec: Vec<_> = self.into();

        vec.into_iter().rev().collect::<SimpleLinkedList<T>>()
    }
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T: Debug + Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut simple_linked_list = SimpleLinkedList::new();

        for i in iter {
            simple_linked_list.push(i);
        }

        simple_linked_list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut current_node = self.head.map(|val| val);
        let mut result = vec!();

        while current_node.is_some() {
            let curr_node = current_node.unwrap();
            result.push(curr_node.data);
            current_node = curr_node.next;
        }

        result.into_iter().rev().collect()
    }
}