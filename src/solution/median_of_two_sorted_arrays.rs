use super::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_size = nums1.len() + nums2.len();
        let mut tmp = Vec::<i32>::with_capacity(total_size / 2 + 1);
        let mut idx = 0;

        let mut left = nums1.into_iter().peekable();
        let mut right = nums2.into_iter().peekable();
        let size = tmp.capacity();
        while idx < size {
            idx += 1;
            match (left.peek(), right.peek()) {
                (None, None) => unreachable!(),
                (None, Some(_r)) => tmp.push(right.next().unwrap()),
                (Some(_l), None) => tmp.push(left.next().unwrap()),
                (Some(l), Some(r)) => {
                    if l <= r {
                        tmp.push(left.next().unwrap())
                    } else {
                        tmp.push(right.next().unwrap())
                    }
                }
            }
        }
        println!("tmp {:?}", tmp);
        if total_size % 2 == 0 {
            let idx = tmp.len() - 1;
            let res = tmp[idx] + tmp[idx - 1];
            (res as f64) / 2.0
        } else {
            (*tmp.last().unwrap()) as f64
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test1() {
        let res = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(res, 2f64);
    }

    #[test]
    fn test_2() {
        let res = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(2.5, res);
    }

    #[test]
    fn test3() {
        let res = Solution::find_median_sorted_arrays(vec![], vec![1]);
        assert_eq!(1f64, res);
    }

    #[test]
    fn test4() {
        let res = Solution::find_median_sorted_arrays(vec![], vec![2, 3]);
        assert_eq!(2.5f64, res);
    }
}
