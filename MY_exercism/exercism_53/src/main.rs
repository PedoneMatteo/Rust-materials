//Write a prototype of the music player application.
// For the prototype, each song will simply be represented by a number. Given a range of numbers (the song IDs), create a singly linked list.
// Given a singly linked list, you should be able to reverse the list to play the songs in the opposite order.
use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut size = 0;
        while current_node.is_some() {
            size += 1;
            current_node = &(current_node).as_ref()
                .unwrap().next;
        }
        size
    }
/*                                  ALTERNATIVA ESATTA
    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut size = 0;
        while let Some(x) = current_node {
            size += 1;
            current_node = &x.next;
        }
        size
    }*/
    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap();
            self.head = head_node.next;
            Some(head_node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &(head.data))
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ret = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            ret.push(x);
        }
        ret
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut ret = Vec::new();
        while let Some(x) = self.pop() {
            ret.insert(0, x);
        }
        ret
    }
}

fn main() {
    println!("Hello, world!");
}
