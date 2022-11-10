use md5;

pub fn p1(input: &str) -> usize {
    let mut i = 1;
    loop {
        let hex = hex_digest(&format!("{input}{i}"));
        if hex.starts_with("00000") {
            return i as usize;
        }
        i += 1;
    }
}

pub fn p2(input: &str) -> usize {
    let mut i = 1;
    loop {
        let hex = hex_digest(&format!("{input}{i}"));
        if hex.starts_with("000000") {
            return i as usize;
        }
        i += 1;
    }
}

fn hex_digest(s: &str) -> String {
    let digest = md5::compute(s);
    format!("{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(p1("abcdef"), 609043);
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(p2("abcdef"), 6742839);
    }
}
