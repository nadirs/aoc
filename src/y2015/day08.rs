use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char},
    combinator::{map, map_res},
    multi::many0,
    sequence::preceded,
    IResult,
};

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, l| acc + code_len(l) - mem_len(l))
}

pub fn p2(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, l| acc + escaped_len(l) - code_len(l))
}

fn code_len(input: &str) -> usize {
    input.chars().count()
}

fn mem_len(input: &str) -> usize {
    let (_, s) = parse_line(input).unwrap();
    s.chars().count()
}

fn escaped_len(input: &str) -> usize {
    escape_line(input).chars().count()
}

fn parse_line(input: &str) -> IResult<&str, String> {
    let hex_to_char = |src| u8::from_str_radix(src, 16).map(|b| b as char);
    let parse_char = alt((
        map_res(preceded(tag("\\x"), take(2usize)), hex_to_char),
        preceded(tag("\\"), alt((char('"'), char('\\')))),
        anychar,
    ));
    let mut parse_str = map(many0(parse_char), |v| v.into_iter().collect::<String>());

    let input = input.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
    parse_str(input)
}

fn escape_line(input: &str) -> String {
    let escaped: String = input
        .chars()
        .map(|c| {
            if c == '"' {
                "\\\"".into()
            } else if c == '\\' {
                "\\\\".into()
            } else {
                format!("{c}")
            }
        })
        .collect();
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(code_len("\"\""), 2);
        assert_eq!(mem_len("\"\""), 0);
        assert_eq!(code_len("\"abc\""), 5);
        assert_eq!(mem_len("\"abc\""), 3);
        assert_eq!(code_len("\"aaa\\\"aaa\""), 10);
        assert_eq!(mem_len("\"aaa\\\"aaa\""), 7);
        assert_eq!(code_len("\"\\x27\""), 6);
        assert_eq!(mem_len("\"\\x27\""), 1);
        assert_eq!(code_len("\"qsmzhnx\\\"\""), 11);
        assert_eq!(mem_len("\"qsmzhnx\\\"\""), 8);

        assert_eq!(p1("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"\n"), 12)
    }

    #[test]
    fn test_p2() {
        assert_eq!(escaped_len("\"\""), 6);
        assert_eq!(escaped_len("\"abc\""), 9);
        assert_eq!(escaped_len("\"aaa\\\"aaa\""), 16);
        assert_eq!(escaped_len("\"\\x27\""), 11);

        assert_eq!(
            p2("\"\"\n\
                \"abc\"\n\
                \"aaa\\\"aaa\"\n\
                \"\\x27\"\n"),
            19
        );
    }
}
