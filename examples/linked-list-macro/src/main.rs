use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
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

fn main() {
    let my_list = ListNode {
        value: 10,
        next: Some(Box::new(ListNode {
            value: 20,
            next: Some(Box::new(ListNode::new(30))),
        })),
    };

    println!("{}", my_list);
}

fn sum(list: ListNode<usize>) -> usize {
    list.value + list.next.map_or(0, |tail| sum(*tail))
}

#[cfg(test)]
// #[rustfmt::skip]
mod test {

    use super::*;

    #[test]
    fn sum_list() {
        // Sum of [3] is 3
        let list = ListNode {
            value: 3,
            next: None,
        };
        assert_eq!(3, sum(list));

        // Sum of [3, 5] is 8
        let list = ListNode {
            value: 3,
            next: Some(Box::new(ListNode {
                value: 5,
                next: None,
            })),
        };
        assert_eq!(8, sum(list));
    }

    macro_rules! list {
        ($value: expr) => {
            ListNode { value: $value, next: None }
        };
        ($value: expr, $($more_values: expr),+) => {
            ListNode { value: $value, next: Some(Box::new(
                list!($($more_values),+)
            )) }
        }
    }

    macro_rules! make_list {
        ($element:expr) => {
            ListNode {
                value: $element,
                next: None,
            }
        };
    }

    #[test]
    fn other_test() {
        let list = list![1, 2];
    }
}
