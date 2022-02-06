use super::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let str_vec = s.chars().collect::<Vec<_>>();
        let mut set = std::collections::HashSet::<char>::with_capacity(s.len());
        let mut left_idx = 0;
        let mut right_idx = 0;
        let mut max: usize;

        // 在起点尽可能大展开
        while right_idx < s.len() {
            let tmp = str_vec[right_idx];
            if !set.contains(&tmp) {
                right_idx += 1;
                set.insert(tmp);
            } else {
                break;
            }
        }

        max = right_idx - left_idx;

        while right_idx < s.len() {
            let tmp = str_vec[right_idx];
            if !set.contains(&tmp) {
            } else {
                while tmp != str_vec[left_idx] {
                    set.remove(&str_vec[left_idx]);
                    left_idx += 1;
                }
                set.remove(&str_vec[left_idx]);
                left_idx += 1;
            }
            right_idx += 1;
            set.insert(tmp);
            if max < (right_idx - left_idx) {
                max = right_idx - left_idx;
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test() {
        let res = Solution::length_of_longest_substring(String::from("abcabcbb"));

        assert_eq!(3, res)
    }
}
