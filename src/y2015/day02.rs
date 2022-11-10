pub fn p1(input: &str) -> usize {
    input.lines().map(resolve_wrapping).sum()
}

pub fn p2(input: &str) -> usize {
    input.lines().map(resolve_ribbon).sum()
}

fn resolve_wrapping(line: &str) -> usize {
    let (l, w, h) = parse_dimensions(line);
    let [a, b, c] = sorted_dims(l, w, h);

    2 * a * b + 2 * b * c + 2 * a * c + a * b
}

fn resolve_ribbon(line: &str) -> usize {
    let (l, w, h) = parse_dimensions(line);
    let [a, b, c] = sorted_dims(l, w, h);

    2 * (a + b) + a * b * c
}

fn parse_dimensions(line: &str) -> (usize, usize, usize) {
    let data = line
        .split('x')
        .map(|n| n.parse().expect(n))
        .collect::<Vec<_>>();

    (data[0], data[1], data[2])
}

fn sorted_dims(l: usize, w: usize, h: usize) -> [usize; 3] {
    let mut dims = [l, w, h];
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
