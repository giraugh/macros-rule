mod list_node;
use list_node::*;

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

    macro_rules! make_list {
        ($value: expr) => {
            ListNode { value: $value, next: None }
        };
        ($value: expr, $($more_values: expr),+) => {
            ListNode { value: $value, next: Some(Box::new(
                make_list!($($more_values),+)
            )) }
        }
    }

    #[test]
    fn sum_list_with_macro() {
        // Sum of [3] is 3
        let list = make_list![3];
        assert_eq!(3, sum(list));

        // Sum of [3, 5] is 8
        let list = make_list![3, 5];
        assert_eq!(8, sum(list));
    }
}
