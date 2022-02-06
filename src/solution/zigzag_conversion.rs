use std::usize;

use super::Solution;

struct Term {
    hor: Vec<char>,
    link: Vec<Option<char>>,
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let wide = if num_rows == 1 {
            0
        } else {
            num_rows as usize - 2
        };
        let group_size = s.len() / num_rows as usize;
        let extra_size = s.len() % num_rows as usize;

        // println!(
        //     "wide {} group_size {} extra_Size {}",
        //     wide, group_size, extra_size
        // );

        let mut str_vec = s.chars();
        let mut terms = Vec::<Term>::with_capacity(if extra_size == 0 {
            group_size
        } else {
            group_size + 1
        });

        loop {
            if let Some(ch) = str_vec.next() {
                let mut hor = Vec::with_capacity(num_rows as usize);
                hor.push(ch);
                for _ in 1..num_rows {
                    match str_vec.next() {
                        Some(ch) => hor.push(ch),
                        None => break,
                    }
                }
                let link = if wide != 0 {
                    let mut link = Vec::with_capacity(wide);
                    for _ in 0..wide {
                        match str_vec.next() {
                            Some(ch) => link.push(Some(ch)),
                            None => link.push(None),
                        }
                    }
                    link.reverse();
                    link
                } else {
                    vec![]
                };
                terms.push(Term { hor, link })
            } else {
                break;
            }
        }
        let mut st = String::with_capacity(s.len());
        for l in 0..(num_rows as usize) {
            for term in terms.iter() {
                match l {
                    idx if idx == (num_rows as usize - 1) || idx == 0 => {
                        if let Some(c) = term.hor.get(idx) {
                            st.push(*c);
                        }
                    }
                    idx => {
                        if let Some(ch) = term.hor.get(idx) {
                            st.push(*ch)
                        }
                        if let Some(ch) = term.link.get(idx - 1).and_then(|s| s.clone()) {
                            st.push(ch)
                        }
                    }
                }
            }
        }

        st
    }
}

#[cfg(test)]
mod test {
    use crate::solution::Solution;

    #[test]
    fn test1() {
        let s = String::from("PAYPALISHIRING");
        let res = Solution::convert(s, 3);
        assert_eq!(res, "PAHNAPLSIIGYIR")
    }

    #[test]
    fn test2() {
        let s = String::from("PAYPALISHIRING");
        let res = Solution::convert(s, 4);
        assert_eq!(res, "PINALSIGYAHRPI")
    }

    #[test]
    fn test3() {
        let s = String::from("A");
        let res = Solution::convert(s, 1);
        assert_eq!(res, "A")
    }

    #[test]
    fn test4() {
        let s = String::from("ABCDE");
        let res = Solution::convert(s, 4);
        assert_eq!(res, "ABCED")
    }
}
