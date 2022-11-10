use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    input.lines().filter(|l| is_nice_p1(l)).count()
}

pub fn p2(input: &str) -> usize {
    input.lines().filter(|l| is_nice_p2(l)).count()
}

fn is_nice_p1(s: &str) -> bool {
    has_repetitions(s) && !has_naughty_pairs(s) && has_vowels(s, 3)
}

fn has_repetitions(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
}

fn has_naughty_pairs(s: &str) -> bool {
    ["ab", "cd", "pq", "xy"].iter().any(|cc| s.contains(cc))
}

fn has_vowels(s: &str, n: usize) -> bool {
    s.chars().filter(|c| "aeiou".contains(*c)).count() >= n
}

fn is_nice_p2(s: &str) -> bool {
    let mut set = HashSet::new();

    let mut has_duplicate_pair = false;
    let mut has_xyx = false;
    let mut preprev_pair = None;
    let mut prev_pair = None;
    for (a, b) in s.chars().zip(s.chars().skip(1)) {
        let curr_pair = Some((a, b));

        if preprev_pair == curr_pair {
            has_duplicate_pair = true;
        }
        if prev_pair != curr_pair {
            if !set.insert((a, b)) {
                has_duplicate_pair = true;
            }
        }

        if let Some((prev_a, _)) = prev_pair {
            if prev_a == b {
                has_xyx = true;
            }
        }
        preprev_pair = prev_pair;
        prev_pair = curr_pair;
    }

    has_duplicate_pair && has_xyx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert!(is_nice_p1("ugknbfddgicrmopn"));
        assert_eq!(
            p1("ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb"),
            2
        );
    }

    #[test]
    fn test_p2() {
        assert!(is_nice_p2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_p2("xxyxx"));
        assert!(is_nice_p2("xxxxyx"));
        assert!(!is_nice_p2("uurcxstgmygtbstg"));
        assert!(!is_nice_p2("ieodomkazucvgmuy"));
        assert_eq!(
            p2("qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy"),
            2
        );
    }
}
