pub fn p1(input: &str) -> usize {
    0
}

pub fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 1);
    }
}

aoc::solve!($YEAR, $DAY, p1, p2);