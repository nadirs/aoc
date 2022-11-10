pub fn d01(input: &str) -> isize {
    input.chars().map(parens_to_value).sum()
}

pub fn d01_bis(input: &str) -> usize {
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
    fn test_d1() {
        assert_eq!(d01("(())"), 0);
        assert_eq!(d01("()()"), 0);
        assert_eq!(d01("))((((("), 3);
        assert_eq!(d01("())"), -1);
        assert_eq!(d01("))("), -1);
        assert_eq!(d01(")))"), -3);
        assert_eq!(d01(")())())"), -3);
    }
    #[test]
    fn test_d1_bis() {
        assert_eq!(d01_bis(")"), 1);
        assert_eq!(d01_bis("()())"), 5);
    }
}
