use std::iter::zip;

pub fn p1(input: &str) -> usize {
    parse_input(input)
        .map(|(t, d)| compute_ways(t, d))
        .fold(1, |acc, x| acc * x)
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut lines = input.trim().lines();
    let times = parse_nums(lines.next().unwrap());
    let distances = parse_nums(lines.next().unwrap());

    zip(times, distances)
}

pub fn p2(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let parse_num = |l: &str| {
        l.chars()
            .skip_while(|c| !c.is_ascii_digit())
            .filter(|c| c.is_ascii_digit())
            .fold(0, |acc, c| acc * 10 + (c as u8 - b'0') as usize)
    };
    let time = parse_num(lines.next().unwrap());
    let distance = parse_num(lines.next().unwrap());

    compute_ways(time, distance)
}

fn parse_nums(l: &str) -> impl Iterator<Item = usize> + '_ {
    l.trim()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
}

fn compute_ways(time: usize, distance: usize) -> usize {
    let mut skip = 0;

    for t in 1..time {
        if t * (time - t) > distance {
            break;
        }
        skip += 1;
    }

    time - 1 - skip * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 288);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 71503);
    }

    #[test]
    fn test_compute_ways() {
        assert_eq!(
            parse_input(INPUT)
                .map(|(t, d)| compute_ways(t, d))
                .collect::<Vec<_>>(),
            vec![4, 8, 9]
        );
    }
}

aoc::solve!(2023, 6, p1, p2);
