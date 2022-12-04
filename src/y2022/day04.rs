use std::ops::RangeInclusive;

pub fn p1(input: &str) -> usize {
    solve(input, either_fully_contains_other)
}

pub fn p2(input: &str) -> usize {
    solve(input, are_overlapping)
}

type Pair = RangeInclusive<usize>;

fn solve(input: &str, predicate: fn(&(Pair, Pair)) -> bool) -> usize {
    input
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(',').unwrap();
            (parse_range(first), parse_range(second))
        })
        .filter(predicate)
        .count()
}

fn parse_range(s: &str) -> Pair {
    let (start, end) = s.split_once('-').unwrap();

    (start.parse().unwrap())..=(end.parse().unwrap())
}

fn either_fully_contains_other((l, r): &(Pair, Pair)) -> bool {
    fully_contains(l, r) || fully_contains(r, l)
}

fn fully_contains(l: &Pair, r: &Pair) -> bool {
    l.contains(r.start()) && l.contains(r.end())
}

fn are_overlapping((l, r): &(Pair, Pair)) -> bool {
    l.contains(r.start()) || l.contains(r.end()) || r.contains(l.start()) || r.contains(l.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 4);
    }
}
