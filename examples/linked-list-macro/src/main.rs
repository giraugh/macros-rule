use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }

    pub fn make_pair_list(a: T, b: T) -> Self {
        LinkedListNode {
            value: a,
            next: Some(Box::new(LinkedListNode {
                value: b,
                next: None,
            })),
        }
    }

    pub fn make_triple_list(a: T, b: T, c: T) -> Self {
        LinkedListNode {
            value: a,
            next: Some(Box::new(LinkedListNode {
                value: b,
                next: Some(Box::new(LinkedListNode {
                    value: c,
                    next: None,
                })),
            })),
        }
    }

    pub fn from_slice(values: [T]) -> Self {
        let mut current: Option<LinkedListNode<T>> = None;
        for value in values.into_iter() {
            current = Some(LinkedListNode {
                value,
                next: current.map(Box::new),
            });
        }

        current.unwrap()
    }
}

impl<T: Display> Display for LinkedListNode<T> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.value)?;
        if let Some(node) = &self.next {
            write!(fmt, " -> ")?;
            node.fmt(fmt)
        } else {
            Ok(())
        }
    }
}

fn main() {
    let my_list = LinkedListNode {
        value: 10,
        next: Some(Box::new(LinkedListNode {
            value: 20,
            next: Some(Box::new(LinkedListNode::new(30))),
        })),
    };

    println!("{}", my_list);
}
