use advent_of_code_2022::read_to_lines;

fn main() {
    let lines = read_to_lines("inputs/day13.txt");
    let mut sum = 0;
    for (i, l) in lines.chunks(3).enumerate() {
        let left = &l[0];
        let right = &l[1];
        if compare(left, right) == Some(true) {
            sum += i + 1;
        }
    }
    println!("{sum}");
    let mut lines: Vec<String> = lines.into_iter().filter(|l| l.len() > 0).collect();
    lines.push("[[2]]".to_string());
    lines.push("[[6]]".to_string());
    lines.sort_by(|a, b| {
        if let Some(res) = compare(a, b) {
            if res {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let mut out = 1;
    for (i, s) in lines.iter().enumerate() {
        if (s == "[[2]]") || (s == "[[6]]") {
            out *= i + 1
        }
    }
    dbg!(out);
}

fn split_next(s: &str) -> (&str, &str) {
    let mut counter = 0;
    let mut ind = 0;
    for c in s.chars() {
        if (c == ',') & (counter == 0) {
            break;
        } else if c == '[' {
            counter += 1
        } else if c == ']' {
            counter -= 1
        }
        ind += 1;
    }
    if ind == s.len() {
        (&s, "")
    } else {
        (&s[..ind], &s[(ind + 1)..])
    }
}

fn compare(left: &str, right: &str) -> Option<bool> {
    // dbg!(left, right);
    let mut lr = &left[1..left.len() - 1];
    let mut rr = &right[1..right.len() - 1];
    let mut ll;
    let mut rl;
    loop {
        if (lr.len() == 0) & (rr.len() == 0) {
            return None;
        } else if lr.len() == 0 {
            return Some(true);
        } else if rr.len() == 0 {
            return Some(false);
        }
        (ll, lr) = split_next(lr);
        (rl, rr) = split_next(rr);
        let vl = ll.parse::<u8>();
        let vr = rl.parse::<u8>();
        match (vl, vr) {
            (Ok(v1), Ok(v2)) => {
                if v1 < v2 {
                    return Some(true);
                } else if v1 > v2 {
                    return Some(false);
                }
            }
            (Ok(_), Err(_)) => {
                if let Some(x) = compare(&format!("[{ll}]"), rl) {
                    return Some(x);
                }
            }
            (Err(_), Ok(_)) => {
                if let Some(x) = compare(ll, &format!("[{rl}]")) {
                    return Some(x);
                };
            }
            (Err(_), Err(_)) => {
                if let Some(x) = compare(ll, rl) {
                    return Some(x);
                };
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_next() {
        assert_eq!(split_next("1,2"), ("1", "2"));
        assert_eq!(split_next("[1],2"), ("[1]", "2"));
        assert_eq!(split_next("[[1],2],2"), ("[[1],2]", "2"));
        assert_eq!(split_next("2,[[1],2]"), ("2", "[[1],2]"));
        assert_eq!(split_next("[2,[[1],2]]]"), ("[2,[[1],2]]]", ""));
        assert_eq!(split_next("1,3,1,1"), ("1", "3,1,1"));
    }

    #[test]
    fn test_matching_bracket() {
        assert_eq!(matched("[]"), true);
        assert_eq!(matched("[[[]]]"), true);
        assert_eq!(matched("[1, []]"), true);
        assert_eq!(matched("1, []"), false);
        assert_eq!(matched("[1], []"), false);
    }

    #[test]
    fn t1() {
        assert_eq!(compare("[1,1,3,1,1]", "[1,1,5,1,1]"), Some(true));
    }

    #[test]
    fn t2() {
        assert_eq!(compare("[[1],[2,3,4]]", "[[1],4]"), Some(true));
    }

    #[test]
    fn t3() {
        assert_eq!(compare("[9]", "[[8,7,6]]"), Some(false));
    }

    #[test]
    fn t4() {
        assert_eq!(compare("[[4,4],4,4]", "[[4,4],4,4,4]"), Some(true));
    }

    #[test]
    fn t5() {
        assert_eq!(compare("[7,7,7,7]", "[7,7,7]"), Some(false));
    }

    #[test]
    fn t6() {
        assert_eq!(compare("[]", "[3]"), Some(true));
    }

    #[test]
    fn t7() {
        assert_eq!(compare("[[[]]]", "[[]]"), Some(false));
    }

    #[test]
    fn t8() {
        assert_eq!(
            compare("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            Some(false)
        );
    }

    #[test]
    fn t9() {
        assert_eq!(
            compare(
                "[[],[[0,[10,1],3,[4]],9,[2,10,4,10]],[5,[0,0,[1,2,7,5]]]]",
                "[[[],[[8,5,2,8,0],[4,7,0,5],[2,7,7]],4],[4],[2,8,[[9,4,8]],6,10],[[[3,0,2,5],[],4,[7,9],[1,5,10,9,6]],4],[]]"),
            Some(false)
        );
    }
}
