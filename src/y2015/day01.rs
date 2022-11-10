pub fn p1(input: &str) -> isize {
    input.chars().map(parens_to_value).sum()
}

pub fn p2(input: &str) -> usize {
    let mut floor = 0;
    for (i, n) in input.chars().map(parens_to_value).enumerate() {
        floor += n;
        if floor == -1 {
            return i + 1;
        }
    }
    unreachable!()
}

fn parens_to_value(c: char) -> isize {
    match c {
        '(' => 1,
        ')' => -1,
        _ => {
            println!("invalid char {c}");
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("(())"), 0);
        assert_eq!(p1("()()"), 0);
        assert_eq!(p1("))((((("), 3);
        assert_eq!(p1("())"), -1);
        assert_eq!(p1("))("), -1);
        assert_eq!(p1(")))"), -3);
        assert_eq!(p1(")())())"), -3);
    }
    #[test]
    fn test_p2() {
        assert_eq!(p2(")"), 1);
        assert_eq!(p2("()())"), 5);
    }
}
