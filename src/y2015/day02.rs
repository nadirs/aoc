pub fn p1(input: &str) -> usize {
    sum_calc(input, wrapping_formula)
}

pub fn p2(input: &str) -> usize {
    sum_calc(input, ribbon_formula)
}

fn sum_calc(input: &str, calc_fn: fn([usize; 3]) -> usize) -> usize {
    input
        .lines()
        .map(parse_dimensions)
        .map(sort_dimensions)
        .map(calc_fn)
        .sum()
}

fn wrapping_formula([a, b, c]: [usize; 3]) -> usize {
    2 * a * b + 2 * b * c + 2 * a * c + a * b
}

fn ribbon_formula([a, b, c]: [usize; 3]) -> usize {
    2 * (a + b) + a * b * c
}

fn parse_dimensions(line: &str) -> [usize; 3] {
    let data = line
        .split('x')
        .map(|n| n.parse().expect(n))
        .collect::<Vec<_>>();

    [data[0], data[1], data[2]]
}

fn sort_dimensions(mut dims: [usize; 3]) -> [usize; 3] {
    dims.sort();
    dims
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("2x3x4"), 58);
        assert_eq!(p1("1x1x10"), 43);
        assert_eq!(p1("2x3x4\n1x1x10"), 58 + 43);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("2x3x4"), 34);
        assert_eq!(p2("1x1x10"), 14);
        assert_eq!(p2("2x3x4\n1x1x10"), 34 + 14);
    }
}
