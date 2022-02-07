use super::Solution;

#[derive(Debug)]
struct Ran {
    left: usize,
    right: usize,
    size: usize,
}

impl Ran {
    fn new(height: &[i32]) -> Self {
        let h = height[0].min(height[1]) as usize;
        let size = h;
        Self {
            left: 0,
            right: 1,
            size,
        }
    }
    fn update_size(&mut self, left: usize, right: usize, height: &[i32]) {
        let h = height[left].min(height[right]) as usize;
        let size = h * (right - left);
        if self.size <= size {
            self.right = right;
            self.left = left;
            self.size = size
        }
    }
}

impl Solution {
    /// 从左右端点开始向中间逼近
    /// 每次逼近时保留较高一边
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut max = 0usize;

        while left < right {
            let h = i32::min(height[left], height[right]) as usize;
            let size = h * (right - left);

            if size > max {
                max = size
            }

            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test1() {
        let res = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(res, 49)
    }

    #[test]
    fn test2() {
        let res = Solution::max_area(vec![1, 1]);
        assert_eq!(res, 1)
    }

    #[test]
    fn test3() {
        let res = Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]);
        assert_eq!(res, 17)
    }

    #[test]
    fn test4() {
        let res = Solution::max_area(vec![1, 0, 0, 0, 0, 0, 0, 2, 2]);
        assert_eq!(res, 8)
    }

    #[test]
    fn test5() {
        let res = Solution::max_area(vec![0, 14, 6, 2, 10, 9, 4, 1, 10, 3]);
        assert_eq!(res, 70)
    }
}
