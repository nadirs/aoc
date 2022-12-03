use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = (
                line.chars()
                    .take(line.chars().count() / 2)
                    .collect::<HashSet<char>>(),
                line.chars()
                    .skip(line.chars().count() / 2)
                    .collect::<HashSet<char>>(),
            );

            let common = *left.intersection(&right).next().unwrap();

            score(common)
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    lines
        .chunks(3)
        .map(|chunk| {
            let mut sets = chunk.iter().map(|s| s.chars().collect::<HashSet<_>>());
            let first = sets.next().unwrap();

            let common = *sets
                .fold(first, |acc, other| {
                    acc.intersection(&other).cloned().collect::<HashSet<char>>()
                })
                .iter()
                .next()
                .unwrap();

            score(common)
        })
        .sum()
}

fn score(c: char) -> usize {
    if ('a'..='z').contains(&c) {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 157);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 70);
    }
}
