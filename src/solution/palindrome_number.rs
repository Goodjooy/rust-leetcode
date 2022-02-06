use std::collections::LinkedList;

use super::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut dqueue = LinkedList::<u8>::new();

        if x < 0 {
            false
        } else if x == 0 {
            true
        } else {
            let mut local = x as u32;
            while local > 0 {
                let now = local % 10;
                local = local / 10;
                dqueue.push_front(now as u8);
            }

            let mut result = true;
            while dqueue.len() > 1 {
                match (dqueue.pop_front(), dqueue.pop_back()) {
                    (None, Some(_)) | (Some(_), None) | (None, None) => {
                        result = false;
                        break;
                    }
                    (Some(l), Some(r)) => {
                        if l != r {
                            result = false;
                            break;
                        }
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solution;

    #[test]
    fn test1() {
        let res = solution::Solution::is_palindrome(121);
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = solution::Solution::is_palindrome(-121);
        assert_eq!(res, false);
    }
}
