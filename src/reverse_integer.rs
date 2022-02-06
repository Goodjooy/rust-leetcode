use crate::Solution;
use std::collections::LinkedList;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut stack = LinkedList::<u8>::new();
        let mut local = x;
        while local >= 10 || local <= -10 {
            let now = (local % 10).abs() as u8;
            local = local / 10;
            stack.push_back(now);
        }

        let sign = if local < 0 { -1 } else { 1 };
        stack.push_back(local.abs() as u8);

        // println!("Stack {:?}", stack);

        let mut result = 0i32;

        while let Some(i) = stack.pop_front() {
            match result
                .checked_mul(10i32)
                .and_then(|n| n.checked_add(i as i32))
            {
                Some(i) => result = i,
                None => return 0,
            }
        }

        match result.checked_mul(sign) {
            Some(i) => i,
            None => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let res = Solution::reverse(123);
        assert_eq!(321, res);
    }

    #[test]
    fn test2() {
        let res = Solution::reverse(-123);
        assert_eq!(-321, res);
    }
    #[test]
    fn test3() {
        let res = Solution::reverse(120);
        assert_eq!(21, res);
    }

}
