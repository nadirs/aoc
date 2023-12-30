use std::collections::{BTreeSet, VecDeque};

pub fn p1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(parse_line_wins)
        .map(|wins| {
            if wins > 0 {
                2usize.pow(wins as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    let mut acc = VecDeque::new();
    let mut total = 0;

    for wins in input.trim().lines().map(parse_line_wins) {
        let multiplier = acc.pop_front().unwrap_or(1);

        total += multiplier;

        for w in 0..wins {
            if w < acc.len() {
                acc[w] += multiplier;
            } else {
                acc.push_back(multiplier + 1);
            }
        }
    }

    total
}

fn parse_line_wins(l: &str) -> usize {
    let (good_nums, my_nums) = l.split_once(": ").unwrap().1.split_once(" | ").unwrap();

    let good_nums: BTreeSet<usize> = parse_nums(good_nums).collect();
    parse_nums(my_nums)
        .filter(|n| good_nums.contains(n))
        .count()
}

fn parse_nums(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 30);
    }
}

aoc::solve!(2023, 4, p1, p2);
