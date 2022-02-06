pub use crate::solution::Solution;

enum Status {
    Start,
    Sign,
    Digit,
    Others,
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut str_iter = s.chars();
        let mut status = Status::Start;
        let mut result = 0i32;
        let mut sign = 1i8;

        while let Some(ch) = str_iter.next() {
            match status {
                Status::Start => {
                    if ch.is_whitespace() {
                    } else if ch.is_digit(10) {
                        result = match result
                            .checked_mul(10)
                            .and_then(|n| n.checked_add(ch.to_digit(10).unwrap() as i32))
                        {
                            Some(ok) => {
                                status = Status::Digit;
                                ok
                            }
                            None => {
                                status = Status::Others;
                                if sign == -1 {
                                    i32::MIN
                                } else {
                                    i32::MAX
                                }
                            }
                        };
                    } else if ch == '-' || ch == '+' {
                        sign = if ch == '-' { -1 } else { 1 };
                        status = Status::Sign;
                    } else {
                        status = Status::Others;
                    }
                }
                Status::Digit | Status::Sign => {
                    if let Some(n) = ch.to_digit(10) {
                        result = match (result)
                            .checked_mul(10)
                            .and_then(|num| num.checked_add(n as i32))
                        {
                            Some(i) => {
                                status = Status::Digit;
                                i
                            }
                            None => {
                                status = Status::Others;
                                if sign == -1 {
                                    i32::MIN
                                } else {
                                    i32::MAX
                                }
                            }
                        };
                    } else {
                        status = Status::Others
                    }
                }
                Status::Others => break,
            }
        }

        match result.checked_mul(sign as i32) {
            Some(i) => i,
            None => {
                if sign == -1 {
                    i32::MIN
                } else {
                    i32::MAX
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::my_atoi("42".to_string());
        assert_eq!(res, 42)
    }

    #[test]
    fn test2() {
        let res = Solution::my_atoi("   -42".to_string());
        assert_eq!(res, -42)
    }

    #[test]
    fn test3() {
        let res = Solution::my_atoi("4193 with word".to_string());
        assert_eq!(res, 4193)
    }

    #[test]
    fn test4() {
        let res = Solution::my_atoi("-91283472332".to_string());
        assert_eq!(res, -2147483648)
    }
}
