use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        let mut iter = strs
            .iter()
            .map(|s| s.chars().peekable())
            .collect::<Vec<_>>();
        loop {
            if iter.iter_mut().any(|f| f.peek().is_none()) {
                break;
            }
            let now = iter
                .iter_mut()
                .filter_map(|f| f.next())
                .collect::<HashSet<_>>();

            if now.len() == 1 {
                res.push(now.into_iter().next().unwrap());
            } else {
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod tst {
    use crate::solution::Solution;

    #[test]
    fn test1() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, "fl")
    }
}
