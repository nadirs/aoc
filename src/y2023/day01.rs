macro_rules! char_to_usize {
    ($x:expr) => {
        $x as usize - '0' as usize
    };
}

const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn p1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let digits = l.chars().filter(|c| c.is_digit(10));
            let first = digits.clone().next().unwrap();
            let last = digits.clone().next_back().unwrap();

            char_to_usize!(first) * 10 + char_to_usize!(last)
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut word_pos = WORDS
                .iter()
                .enumerate()
                .flat_map(|(i, w)| l.match_indices(w).map(move |(pos, _)| (pos, i + 1)))
                .collect::<Vec<_>>();
            word_pos.sort();

            let mut digit_pos = l
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c.is_digit(10) {
                        Some((i, char_to_usize!(c)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            digit_pos.sort();

            let first_word = word_pos.first();
            let first_digit = digit_pos.first();
            let last_word = word_pos.last();
            let last_digit = digit_pos.last();

            fn choose<T: Ord>(cmp: fn(T, T) -> T, a: Option<T>, b: Option<T>) -> T {
                if a.is_none() {
                    b.unwrap()
                } else if b.is_none() {
                    a.unwrap()
                } else {
                    cmp(a.unwrap(), b.unwrap())
                }
            }

            let (_, first) = choose(Ord::min, first_word, first_digit);
            let (_, last) = choose(Ord::max, last_word, last_digit);

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(p1(INPUT), 142);
    }

    #[test]
    fn test_p2() {
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(p2(INPUT), 281);
    }
    #[test]
    fn test_p2_same() {
        const INPUT: &str = "1two1nine1
eightwothreeeight";

        assert_eq!(p2(INPUT), 11 + 88);
    }
}
