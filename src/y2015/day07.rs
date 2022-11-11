use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, map, map_res},
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn p1(input: &str) -> usize {
    let mut emu = Emu::default();

    for line in input.lines() {
        let (_, (op, dst)) = Op::parse(line).unwrap_or_else(|_| panic!("unable to parse {line}"));
        emu.wire(op, dst);
    }

    emu.resolve_wire("a".into()) as usize
}

pub fn p2(input: &str) -> usize {
    let mut emu = Emu::default();

    for line in input.lines() {
        let (_, (op, dst)) = Op::parse(line).unwrap_or_else(|_| panic!("unable to parse {line}"));
        emu.wire(op, dst);
    }

    let a = emu.resolve_wire("a".into());
    emu.wire(Op::Id(a.into()), "b".into());
    emu.reset();

    emu.resolve_wire("a".into()) as usize
}

#[derive(Debug, Default)]
struct Emu {
    logic: HashMap<String, Op>,
    state: HashMap<String, u16>,
}

impl Emu {
    fn reset(&mut self) {
        self.state = HashMap::new()
    }

    fn wire(&mut self, op: Op, dst: String) {
        self.logic.insert(dst, op);
    }

    fn resolve_op(&mut self, op: Op) -> u16 {
        match op {
            Op::Id(src) => self.resolve_wire(src),
            Op::Not(src) => !self.resolve_wire(src),
            Op::And(a, b) => self.resolve_wire(a) & self.resolve_wire(b),
            Op::Or(a, b) => self.resolve_wire(a) | self.resolve_wire(b),
            Op::Lshift(src, offset) => self.resolve_wire(src) << offset,
            Op::Rshift(src, offset) => self.resolve_wire(src) >> offset,
        }
    }

    fn resolve_wire(&mut self, wire: Wire) -> u16 {
        match wire {
            Wire::Lit(n) => n,
            Wire::Str(s) => {
                if let Some(n) = self.state.get(&s) {
                    *n
                } else {
                    let n = self.resolve_op(self.logic[&s].clone());
                    self.state.insert(s, n);
                    n
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Wire {
    Str(String),
    Lit(u16),
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        Self::Str(s.into())
    }
}

impl From<u16> for Wire {
    fn from(n: u16) -> Self {
        Self::Lit(n)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Id(Wire),
    Not(Wire),
    And(Wire, Wire),
    Or(Wire, Wire),
    Lshift(Wire, u16),
    Rshift(Wire, u16),
}

impl Op {
    fn parse(input: &str) -> IResult<&str, (Op, String)> {
        let mut op_parsers = alt((
            all_consuming(map(parse_wire, Op::Id)),
            map(preceded(tag("NOT "), parse_wire), Op::Not),
            parse_binop(parse_wire, " AND ", parse_wire, Op::And),
            parse_binop(parse_wire, " OR ", parse_wire, Op::Or),
            parse_binop(parse_wire, " LSHIFT ", parse_u16, Op::Lshift),
            parse_binop(parse_wire, " RSHIFT ", parse_u16, Op::Rshift),
        ));
        let parse_op = separated_pair(take_until(" -> "), tag(" -> "), parse_string);
        let (input, (op_input, dst)) = all_consuming(parse_op)(input)?;
        let (_, op) = op_parsers(op_input)?;

        Ok((input, (op, dst)))
    }
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    alt((map(parse_u16, Wire::Lit), map(parse_string, Wire::Str)))(input)
}

fn parse_u16(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse)(input)
}

fn parse_string(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from)(input)
}

fn parse_binop<'a, A, B>(
    left: impl FnMut(&'a str) -> IResult<&'a str, A>,
    sep: &'a str,
    right: impl FnMut(&'a str) -> IResult<&'a str, B>,
    mut f: impl FnMut(A, B) -> Op,
) -> impl FnMut(&'a str) -> IResult<&'a str, Op> {
    map(separated_pair(left, tag(sep), right), move |(a, b)| f(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_parse() {
        assert_eq!(
            Op::parse("NOT x -> h").unwrap().1,
            (Op::Not("x".into()), "h".into())
        );

        assert_eq!(
            Op::parse("123 -> x").unwrap().1,
            (Op::Id(123.into()), "x".into())
        );
        assert_eq!(
            Op::parse("456 -> y").unwrap().1,
            (Op::Id(456.into()), "y".into())
        );
        assert_eq!(
            Op::parse("x AND y -> d").unwrap().1,
            (Op::And("x".into(), "y".into()), "d".into())
        );
        assert_eq!(
            Op::parse("x OR y -> e").unwrap().1,
            (Op::Or("x".into(), "y".into()), "e".into())
        );
        assert_eq!(
            Op::parse("x LSHIFT 2 -> f").unwrap().1,
            (Op::Lshift("x".into(), 2), "f".into())
        );
        assert_eq!(
            Op::parse("y RSHIFT 2 -> g").unwrap().1,
            (Op::Rshift("y".into(), 2), "g".into())
        );
        assert_eq!(
            Op::parse("NOT x -> h").unwrap().1,
            (Op::Not("x".into()), "h".into())
        );
        assert_eq!(
            Op::parse("NOT y -> i").unwrap().1,
            (Op::Not("y".into()), "i".into())
        );

        assert_eq!(
            Op::parse("1 AND fi -> fj").unwrap().1,
            (Op::And((1).into(), "fi".into()), "fj".into())
        );
    }

    #[test]
    fn test_emu() {
        let mut emu = Emu::default();
        emu.wire(Op::Id(123.into()), "x".into());
        emu.wire(Op::Id(456.into()), "y".into());
        emu.wire(Op::And("x".into(), "y".into()), "d".into());
        emu.wire(Op::Or("x".into(), "y".into()), "e".into());
        emu.wire(Op::Lshift("x".into(), 2), "f".into());
        emu.wire(Op::Rshift("y".into(), 2), "g".into());
        emu.wire(Op::Not("x".into()), "h".into());
        emu.wire(Op::Not("y".into()), "i".into());

        assert_eq!(emu.resolve_wire("d".into()), 72);
        assert_eq!(emu.resolve_wire("e".into()), 507);
        assert_eq!(emu.resolve_wire("f".into()), 492);
        assert_eq!(emu.resolve_wire("g".into()), 114);
        assert_eq!(emu.resolve_wire("h".into()), 65412);
        assert_eq!(emu.resolve_wire("i".into()), 65079);
        assert_eq!(emu.resolve_wire("x".into()), 123);
        assert_eq!(emu.resolve_wire("y".into()), 456);
    }

    #[test]
    fn test_p1() {
        let input = "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> a\n\
            NOT y -> i\n";

        assert_eq!(p1(input), 65412);
    }

    #[test]
    fn test_p2() {
        //assert_eq!(p2(""), 2);
    }
}
