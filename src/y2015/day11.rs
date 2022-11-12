use std::collections::HashSet;

pub fn p1(input: &str) -> String {
    next_pass(input)
}

pub fn p2(input: &str) -> String {
    next_pass(&next_pass(input))
}

fn next_pass(s: &str) -> String {
    let mut pass = (s.chars().next().unwrap() as u64) << (7 * 8)
        | (s.chars().nth(1).unwrap() as u64) << (6 * 8)
        | (s.chars().nth(2).unwrap() as u64) << (5 * 8)
        | (s.chars().nth(3).unwrap() as u64) << (4 * 8)
        | (s.chars().nth(4).unwrap() as u64) << (3 * 8)
        | (s.chars().nth(5).unwrap() as u64) << (2 * 8)
        | (s.chars().nth(6).unwrap() as u64) << 8
        | s.chars().nth(7).unwrap() as u64;

    let mut chars: String = gen_chars(pass).collect();
    for _ in 0..1_000_000 {
        if chars == "zzzzzzzz" {
            pass = 0x6161616161616161;
        }

        pass += 1;
        for b in 0..8 {
            if char_at(pass, 7 - b) <= 'z' {
                break;
            }
            pass = (pass & !(0xff << ((b) * 8))) + (('a' as u64 + 0x100) << (b * 8));
        }

        chars = gen_chars(pass).collect();
        if has_3_straights(&chars) && has_2_pairs(&chars) && !has_forbidden_letters(&chars) {
            return chars;
        }
    }

    panic!("too many iterations")
}

fn gen_chars(n: u64) -> impl Iterator<Item = char> {
    (0..8).map(move |b| char_at(n, b))
}

fn char_at(n: u64, b: u64) -> char {
    ((n >> ((7 - b) * 8)) & 0xff) as u8 as char
}

fn has_3_straights(s: &str) -> bool {
    for ((a, b), c) in s.chars().zip(s.chars().skip(1)).zip(s.chars().skip(2)) {
        if b as u8 == a as u8 + 1 && c as u8 == b as u8 + 1 {
            return true;
        }
    }

    false
}

fn has_forbidden_letters(s: &str) -> bool {
    !s.chars()
        .collect::<HashSet<_>>()
        .is_disjoint(&HashSet::from(['i', 'o', 'l']))
}

fn has_2_pairs(s: &str) -> bool {
    let mut first_pair = None;
    for ((i, a), (_, b)) in s.char_indices().zip(s.char_indices().skip(1)) {
        if a == b {
            if let Some(x) = first_pair {
                if x + 1 != i {
                    return true;
                }
            } else {
                first_pair = Some(i);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert!(has_3_straights("hijklmmn"));
        assert!(has_forbidden_letters("hijklmmn"));
        assert!(!has_3_straights("abbceffg"));
        assert!(!has_forbidden_letters("abbceffg"));
        assert!(has_2_pairs("abbceffg"));
        assert!(!has_2_pairs("abbcegjk"));

        assert_eq!(&next_pass("abcdefgh"), "abcdffaa");
    }
}
