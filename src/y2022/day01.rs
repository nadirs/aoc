pub fn p1(input: &str) -> usize {
    sort_elves(parse_elves(input))[0]
}

fn parse_elves(input: &str) -> Vec<usize> {
    input
        .split_terminator("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

fn sort_elves(mut elves: Vec<usize>) -> Vec<usize> {
    elves.sort_by_key(|&n| n as isize * -1);
    elves
}

pub fn p2(input: &str) -> usize {
    sort_elves(parse_elves(input))[0..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 24000);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 45000);
    }
}
