use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            vec![]
        } else {
            let mut res = HashSet::<Vec<i32>>::new();
            let mut sorted = nums;
            sorted.sort();
            println!("{:?}", sorted);

            for idx in 0..sorted.len() - 2 {
                if idx > 0 && sorted[idx - 1] == sorted[idx] {
                    continue;
                }

                let mut left = idx + 1;
                let mut right = sorted.len() - 1;

                let mut sum = sorted[idx] + sorted[left] + sorted[right];

                while left < right {
                    // println!("{} {} {}", idx, left, right);
                    // println!("{} {} {}", sorted[idx], sorted[left], sorted[right]);
                    if sum == 0 {
                        res.insert(vec![sorted[idx], sorted[left], sorted[right]]);
                        right -= 1;
                        left += 1;
                        sum = sorted[idx] + sorted[left] + sorted[right];
                    } else if sum > 0 {
                        right -= 1;
                        sum = sorted[idx] + sorted[left] + sorted[right];
                    } else {
                        left += 1;
                        sum = sorted[idx] + sorted[left] + sorted[right];
                    }
                }
            }
            res.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tset {
    use crate::solution::Solution;

    #[test]
    fn test() {
        let res = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        println!("{:?}", res)
    }
}
