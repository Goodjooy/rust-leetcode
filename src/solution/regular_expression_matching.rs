use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Char {
    Ch(char),
    Any,
}

#[derive(Debug)]
struct NFA {
    status_set: HashSet<usize>,
    start_status: usize,

    trans: HashMap<(usize, Char), usize>,
    nil_trans: HashMap<usize, HashSet<usize>>,

    accept_status: HashSet<usize>,
}

impl NFA {
    fn new(start: usize) -> Self {
        Self {
            status_set: HashSet::from_iter([start]),
            start_status: start,
            trans: HashMap::new(),
            nil_trans: HashMap::new(),
            accept_status: HashSet::new(),
        }
    }

    fn get_init(&self) -> usize {
        self.start_status
    }

    fn add_zero_or_more(&mut self, form: usize, ch: Char) -> usize {
        let mid_status = form + 1;
        let end_status = mid_status + 1;

        self.trans.insert((form, ch), mid_status);
        self.trans.insert((mid_status, ch), mid_status);

        // update form Nil trans
        if let Some(set) = self.nil_trans.get_mut(&form) {
            set.insert(end_status);
        } else {
            self.nil_trans
                .insert(form, HashSet::from_iter([end_status]));
        }

        // add Mid New MIl trans
        self.nil_trans
            .insert(mid_status, HashSet::from_iter([end_status]));

        self.status_set.insert(mid_status);
        self.status_set.insert(end_status);

        end_status
    }

    fn add_normal(&mut self, form: usize, ch: Char) -> usize {
        let end_status = form + 1;

        self.trans.insert((form, ch), end_status);

        self.status_set.insert(end_status);

        end_status
    }

    fn add_accept_status(&mut self, status: usize) {
        self.accept_status.insert(status);
    }
}

impl NFA {
    fn init(&self) -> HashSet<usize> {
        let mut acc = self.nil_next(self.start_status);
        acc.insert(self.start_status);
        acc
    }

    fn next(&self, now: &HashSet<usize>, input: Char) -> HashSet<usize> {
        now.iter()
            .filter_map(|f| {
                self.trans
                    .get(&(*f, input))
                    .or_else(|| self.trans.get(&(*f, Char::Any)))
            })
            .map(|s| (*s, self.nil_next(*s)))
            .map(|(s, mut set)| {
                set.insert(s);
                set
            })
            .reduce(|l, r| HashSet::from_iter(l.union(&r).into_iter().cloned()))
            .unwrap_or_default()
    }

    fn nil_next(&self, now: usize) -> HashSet<usize> {
        if let Some(set) = self.nil_trans.get(&now) {
            set.iter()
                .map(|f| *f)
                .map(|s| (s, self.nil_next(s)))
                .map(|(s, mut set)| {
                    set.insert(s);
                    set
                })
                .reduce(|l, r| HashSet::from_iter(l.union(&r).into_iter().cloned()))
                .unwrap_or(HashSet::from_iter([now]))
        } else {
            HashSet::from_iter([now])
        }
    }

    fn is_accept(&self, now: &HashSet<usize>) -> bool {
        !self.accept_status.is_disjoint(now)
    }
}

impl NFA {
    fn load_from_pattern(pattern: &str) -> Self {
        let mut result = Self::new(0);

        let mut peek_str = pattern.chars().peekable();

        let mut form = 0;

        while let Some(ch) = peek_str.peek() {
            let c = match ch {
                c if c == &'.' => {
                    peek_str.next().unwrap();
                    Char::Any
                }
                _ => Char::Ch(peek_str.next().unwrap()),
            };

            if let Some(c_next) = peek_str.peek() {
                match c_next {
                    cn if cn == &'*' => {
                        peek_str.next().unwrap();
                        form = result.add_zero_or_more(form, c);
                    }
                    _ => {
                        form = result.add_normal(form, c);
                    }
                }
            } else {
                form = result.add_normal(form, c);
            }
        }

        result.add_accept_status(form);
        result
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let nfa = NFA::load_from_pattern(&p);

        let mut now = nfa.init();
        let mut s = s.chars();
        while let Some(ch) = s.next() {
            if now.is_empty() {
                return false;
            }
            now = nfa.next(&now, Char::Ch(ch))
        }

        nfa.is_accept(&now)
    }
}

#[cfg(test)]
mod test_reg {
    use crate::solution::Solution;

    #[test]
    fn test1() {
        let s = "aa".to_string();
        let p = "a".to_string();

        let res = Solution::is_match(s, p);
        assert_eq!(res, false)
    }

    #[test]
    fn test2() {
        let s = "aa".to_string();
        let p = "a*".to_string();

        let res = Solution::is_match(s, p);
        assert_eq!(res, true)
    }

    #[test]
    fn test3() {
        let s = "ab".to_string();
        let p = ".*".to_string();

        let res = Solution::is_match(s, p);
        assert_eq!(res, true)
    }

    #[test]
    fn test4() {
        let s = "aab".to_string();
        let p = "c*a*b".to_string();

        let res = Solution::is_match(s, p);
        assert_eq!(res, true)
    }

    #[test]
    fn test5() {
        let s = "aaa".to_string();
        let p = "a*a".to_string();

        let res = Solution::is_match(s, p);
        assert_eq!(res, true)
    }
}
