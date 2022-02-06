use super::Solution;

#[derive(Debug, PartialEq, Eq)]
enum Center {
    Single(usize),
    Double(usize, usize),
}

impl Center {
    fn get_ends(&self, wide: usize) -> Option<(usize, usize)> {
        match self {
            Center::Double(idx, _) | Center::Single(idx) if idx.checked_sub(wide).is_none() => None,
            Center::Single(idx) => Some((idx - wide, idx + wide)),
            Center::Double(l, r) => Some((l - wide, r + wide)),
        }
    }

    fn check_ends_expand(&self, wide: usize, src: &[char]) -> bool {
        if let Some((l, r)) = self.get_ends(wide) {
            src.get(l) == src.get(r)
        } else {
            false
        }
    }

    fn check_to_ends(&self, wide: usize, src: &[char]) -> bool {
        if let Some((l, r)) = self.get_ends(wide) {
            if r >= src.len() {
                return false;
            }
            match self {
                Center::Single(idx) => (l..=(*idx))
                    .zip(((*idx)..=r).rev())
                    .all(|(l, r)| src.get(l) == src.get(r)),
                Center::Double(lc, rc) => (l..=(*lc))
                    .zip(((*rc)..=r).rev())
                    .all(|(l, r)| src.get(l) == src.get(r)),
            }
        } else {
            false
        }
    }

    fn get_sub_str<'s>(&self, wide: usize, s: &'s String) -> Option<&'s str> {
        match self.get_ends(wide) {
            Some((l, r)) => Some(&s[l..=r]),
            None => None,
        }
    }

    fn try_right_expand(self, src: &[char]) -> (Self, bool) {
        match self {
            Center::Single(idx) => {
                if src.get(idx) == src.get(idx + 1) {
                    (Center::Double(idx, idx + 1), true)
                } else {
                    (Center::Single(idx), false)
                }
            }
            d => (d, false),
        }
    }

    fn move_right(self) -> Self {
        match self {
            Center::Single(s) => Center::Single(s + 1),
            Center::Double(_l, r) => Self::Single(r),
        }
    }
}

#[cfg(test)]
mod test_center {
    use super::Center;

    #[test]
    fn test_check() {
        let v = vec!['a', 'b', 'c', 'b', 'a'];
        let center = Center::Single(2);

        assert!(center.check_to_ends(1, &v))
    }

    #[test]
    fn test_ends() {
        let center = Center::Single(2);
        let res = center.get_ends(2);
        assert_eq!(res, Some((0, 4)));
        let res = center.get_ends(4);
        assert_eq!(res, None);
    }

    #[test]
    fn test_str() {
        let s = String::from("abcba");
        let center = Center::Single(2);
        let st = center.get_sub_str(2, &s).unwrap();

        assert_eq!(st, s);
    }

    #[test]
    fn try_left_expand() {
        let v = vec!['a', 'b', 'b', 'a', 'a'];
        let center = Center::Single(1);
        let (center, expend) = center.try_right_expand(&v);

        assert_eq!(expend, true);
        assert_eq!(center, Center::Double(1, 2))
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            s
        } else {
            let str_vec = s.chars().collect::<Vec<_>>();
            let mut center = Center::Single(0);
            let mut wide = 0usize;
            let mut result: &str = &s[0..1];

            while Self::in_range(&center, wide, str_vec.len()) {
                // 首先尝试左右扩展
                if center.check_ends_expand(wide+1, &str_vec) {
                    wide += 1;
                    let sub = center.get_sub_str(wide, &s).unwrap();
                    if sub.len() > result.len() {
                        result = sub;
                    }
                } else {
                    // 尝试 向右边扩展
                    let (c, expand) = center.try_right_expand(&str_vec);
                    center = c;
                    if expand {
                        if !center.check_to_ends(wide, &str_vec) {
                            wide = 0;
                        }
                        let sub = center.get_sub_str(wide, &s).unwrap();
                        if sub.len() > result.len() {
                            result = sub;
                        }
                        continue;
                    }
                    // 尝试向右边移动
                    center = center.move_right();
                    if !center.check_to_ends(wide, &str_vec) {
                        wide = 0;
                    }
                }
            }

            result.to_string()
        }
    }

    fn in_range(center: &Center, wide: usize, total: usize) -> bool {
        match center {
            Center::Double(_, idx) | Center::Single(idx) => idx + wide < total,
        }
    }
}

#[cfg(test)]
mod test_str {
  

    use crate::solution::Solution;
    #[test]

fn test1() {
        let s = String::from("babad");
        let res = Solution::longest_palindrome(s);
        assert_eq!(res, "bab")
    }

    #[test]
    fn test2() {
        let s = String::from("cbbd");
        let res = Solution::longest_palindrome(s);
        assert_eq!(res, "bb")
    }
}
