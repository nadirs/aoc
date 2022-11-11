use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, map, map_res},
    sequence::separated_pair,
    IResult,
};

pub fn p1(input: &str) -> usize {
    let mut emu = Emu::default();

    for line in input.lines() {
        let (_, (op, dst)) = Op::parse(line).expect(&format!("unable to parse {line}"));
        emu.wire(op, dst);
    }

    emu.resolve_wire("a".into()) as usize
}

pub fn p2(input: &str) -> usize {
    let mut emu = Emu::default();

    for line in input.lines() {
        let (_, (op, dst)) = Op::parse(line).expect(&format!("unable to parse {line}"));
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

    fn wire(&mut self, op: Op, dst: Dest) {
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

type Dest = String;

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
    fn parse(input: &str) -> IResult<&str, (Op, Dest)> {
        let mut op_parsers = alt((
            all_consuming(parse_lit),
            all_consuming(parse_not),
            all_consuming(parse_and),
            all_consuming(parse_or),
            all_consuming(parse_lshift),
            all_consuming(parse_rshift),
        ));
        let parse_op = separated_pair(take_until(" -> "), tag(" -> "), parse_string);
        let (input, (op_input, dst)) = all_consuming(parse_op)(input)?;
        let (_, op) = op_parsers(op_input)?;

        Ok((input, (op, dst)))
    }
}

fn parse_lit(input: &str) -> IResult<&str, Op> {
    map(parse_wire, Op::Id)(input)
}

fn parse_not(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("NOT ")(input)?;
    let (input, src) = parse_wire(input)?;

    Ok((input, Op::Not(src)))
}

fn parse_and(input: &str) -> IResult<&str, Op> {
    let (input, (a, b)) = separated_pair(parse_wire, tag(" AND "), parse_wire)(input)?;

    Ok((input, Op::And(a, b)))
}

fn parse_or(input: &str) -> IResult<&str, Op> {
    let (input, (a, b)) = separated_pair(parse_wire, tag(" OR "), parse_wire)(input)?;

    Ok((input, Op::Or(a, b)))
}

fn parse_lshift(input: &str) -> IResult<&str, Op> {
    let (input, (src, offset)) = separated_pair(parse_wire, tag(" LSHIFT "), parse_u16)(input)?;

    Ok((input, Op::Lshift(src, offset)))
}

fn parse_rshift(input: &str) -> IResult<&str, Op> {
    let (input, (src, offset)) = separated_pair(parse_wire, tag(" RSHIFT "), parse_u16)(input)?;

    Ok((input, Op::Rshift(src, offset)))
}

fn parse_u16(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse)(input)
}

fn parse_string(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from)(input)
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    alt((
        map(parse_u16, Wire::Lit),
        map(alpha1, |s: &str| Wire::Str(s.into())),
    ))(input)
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
