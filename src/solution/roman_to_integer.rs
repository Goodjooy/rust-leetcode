use super::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut str_iter = s.chars().peekable();

        let mut init = 0;

        while let Some(s) = str_iter.peek() {
            match *s {
                'I' => {
                    str_iter.next().unwrap();
                    if let Some(nc) = str_iter.peek() {
                        if nc == &'X' {
                            str_iter.next().unwrap();
                            init += 9;
                        } else if nc == &'V' {
                            str_iter.next().unwrap();
                            init += 5
                        } else {
                            init += 1;
                        }
                    } else {
                        init += 1;
                    }
                }
                'V' => {
                    str_iter.next().unwrap();
                    init += 5;
                }
                'X' => {
                    str_iter.next().unwrap();
                    if let Some(nc) = str_iter.peek() {
                        if nc == &'L' {
                            str_iter.next().unwrap();
                            init += 40;
                        } else if nc == &'C' {
                            str_iter.next().unwrap();
                            init += 90
                        } else {
                            init += 10;
                        }
                    } else {
                        init += 10;
                    }
                }
                'L' => {
                    str_iter.next().unwrap();
                    init += 50;
                }
                'C' => {
                    str_iter.next().unwrap();
                    if let Some(nc) = str_iter.peek() {
                        if nc == &'D' {
                            str_iter.next().unwrap();
                            init += 400;
                        } else if nc == &'M' {
                            str_iter.next().unwrap();
                            init += 900
                        } else {
                            init += 100
                        }
                    } else {
                        init += 100
                    }
                }
                'D' => {
                    str_iter.next().unwrap();
                    init += 500;
                }
                'M' => {
                    str_iter.next().unwrap();
                    init += 1000;
                }

                _ => unreachable!(),
            }
        }

        init
    }
}
