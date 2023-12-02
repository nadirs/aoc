pub fn p1(input: &str) -> usize {
    0
}

pub fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 1);
    }
}
