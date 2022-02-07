use super::Solution;

const ONES: [char; 4] = ['I', 'X', 'C', 'M'];
const FIVES: [char; 3] = ['V', 'L', 'D'];

enum Num {
    Nine,
    OM(u8),
    Four,
    M(u8),
}

impl Num {
    fn to_str(&self, level: usize) -> String {
        let mut st = String::with_capacity(4);
        match self {
            Num::Nine => {
                st.push(ONES[level]);
                st.push(ONES[level + 1])
            }
            Num::OM(n) => {
                st.push(FIVES[level]);
                (0..*n).into_iter().for_each(|_| st.push(ONES[level]));
            }
            Num::Four => {
                st.push(ONES[level]);
                st.push(FIVES[level]);
            }
            Num::M(n) => (0..*n).into_iter().for_each(|_| st.push(ONES[level])),
        }

        st
    }
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut tmp = Vec::with_capacity(4);

        let mut local = num;

        while local > 0 {
            let now = local % 10;
            local = local / 10;
            let num = match now as u8 {
                n if n < 4 => Num::M(n),
                n if n == 4 => Num::Four,
                n if 4 < n && n < 9 => Num::OM(n - 5),
                n if n == 9 => Num::Nine,
                _ => unreachable!(),
            };
            tmp.push(num);
        }

        let mut result = String::with_capacity(16);

        for (idx, num) in tmp.into_iter().enumerate().rev() {
            result.push_str(&num.to_str(idx))
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test1() {
        let res = Solution::int_to_roman(3);
        assert_eq!(res, "III")
    }

    #[test]
    fn test2() {
        let res = Solution::int_to_roman(58);
        assert_eq!(res, "LVIII")
    }

    #[test]
    fn test3() {
        let res = Solution::int_to_roman(1994);
        assert_eq!(res, "MCMXCIV")
    }
}
