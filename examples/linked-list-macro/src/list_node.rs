use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

#[allow(unused)]
impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }

    pub fn make_pair_list(a: T, b: T) -> Self {
        ListNode {
            value: a,
            next: Some(Box::new(ListNode {
                value: b,
                next: None,
            })),
        }
    }

    pub fn make_triple_list(a: T, b: T, c: T) -> Self {
        ListNode {
            value: a,
            next: Some(Box::new(ListNode {
                value: b,
                next: Some(Box::new(ListNode {
                    value: c,
                    next: None,
                })),
            })),
        }
    }

    pub fn from_slice(values: &mut [T]) -> Self
    where
        T: Default,
    {
        let mut current: Option<ListNode<T>> = None;
        for value in values.iter_mut() {
            current = Some(ListNode {
                value: std::mem::take(value),
                next: current.map(Box::new),
            });
        }

        current.unwrap()
    }
}

impl<T: Display> Display for ListNode<T> {
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
