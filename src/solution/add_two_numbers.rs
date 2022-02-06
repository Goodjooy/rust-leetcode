use super::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, Some(l)) | (Some(l), None) => {
                let val = l.val;
                let now = val % 10;
                let next = val / 10;

                if next == 0 {
                    return Some(l);
                }

                let ln = match l.next {
                    Some(n) => {
                        let mut list_node = ListNode::new(n.val + next);
                        list_node.next = n.next;
                        Some(Box::new(list_node))
                    }
                    None => {
                        let list_node = ListNode::new(next);
                        Some(Box::new(list_node))
                    }
                };

                let next = Self::add_two_numbers(ln, None);

                let mut node = ListNode::new(now);
                node.next = next;

                Some(Box::new(node))
            }
            (Some(l), Some(r)) => {
                let sum = l.val + r.val;
                let now = sum % 10;
                let next = sum / 10;

                let ln = l.next;
                let rn = r.next;

                let (ln, rn) = match (ln, rn) {
                    (None, None) => (None, Some(Box::new(ListNode::new(next)))),
                    (None, Some(n)) => {
                        let mut list_node = ListNode::new(n.val + next);
                        list_node.next = n.next;
                        (None, Some(Box::new(list_node)))
                    }
                    (Some(n), None) => {
                        let mut list_node = ListNode::new(n.val + next);
                        list_node.next = n.next;
                        (Some(Box::new(list_node)), None)
                    }
                    (l, Some(r)) => {
                        let mut list_node = ListNode::new(r.val + next);
                        list_node.next = r.next;
                        (l, Some(Box::new(list_node)))
                    }
                };
                let next = Self::add_two_numbers(ln, rn);

                let mut node = ListNode::new(now);
                if let Some(next) = next {
                    if next.val == 0 && next.next.is_none() {
                    } else {
                        node.next = Some(next);
                    }
                }

                Some(Box::new(node))
            }
        }
    }
}
