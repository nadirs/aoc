use std::{iter::Skip, str::Chars};

pub fn p1(input: &str) -> usize {
    exec(input, Mode::P1)
}

pub fn p2(input: &str) -> usize {
    exec(input, Mode::P2)
}

fn exec(input: &str, mode: Mode) -> usize {
    let mut grid = Grid::new(mode);

    for line in input.lines() {
        let op = Op::parse(line);
        grid.exec(op);
    }

    grid.count_lights()
}

const CMD_TURN_ON: &'static str = "turn on";
const CMD_TURN_OFF: &'static str = "turn off";
const CMD_TOGGLE: &'static str = "toggle";

type Pos = (usize, usize);

#[derive(Debug, PartialEq)]
enum OpType {
    On,
    Off,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Op {
    op_type: OpType,
    from: Pos,
    to: Pos,
}

impl Op {
    fn new(op_type: OpType, from: Pos, to: Pos) -> Self {
        Self { op_type, from, to }
    }

    fn parse(s: &str) -> Self {
        let (op_type, parser) = skip_cmd(s);

        let start_coords: String = parser.clone().take_while(|&c| c != ' ').collect();
        let end_coords: String = parser
            .skip(start_coords.len() + " through ".len())
            .take_while(|&c| c != ' ')
            .collect();

        let (x0, y0) = start_coords.split_once(',').expect(&start_coords);
        let (x1, y1) = end_coords.split_once(',').expect(&end_coords);

        Self::new(
            op_type,
            (x0.parse().unwrap(), y0.parse().unwrap()),
            (x1.parse().unwrap(), y1.parse().unwrap()),
        )
    }

    fn exec(&self, n: u8) -> u8 {
        match self.op_type {
            OpType::On => 1,
            OpType::Off => 0,
            OpType::Toggle => (n + 1) % 2,
        }
    }

    fn exec_p2(&self, n: u8) -> u8 {
        match self.op_type {
            OpType::On => n + 1,
            OpType::Off => {
                if n > 1 {
                    n - 1
                } else {
                    0
                }
            }
            OpType::Toggle => n + 2,
        }
    }
}

enum Mode {
    P1,
    P2,
}
struct Grid {
    lights: Vec<u8>,
    mode: Mode,
}
impl Grid {
    fn new(mode: Mode) -> Self {
        Self {
            lights: vec![0; 1_000_000],
            mode,
        }
    }

    fn count_lights(&self) -> usize {
        self.lights.iter().fold(0, |acc, n| acc + *n as usize)
    }

    fn exec(&mut self, op: Op) {
        let (x0, y0) = op.from;
        let (x1, y1) = op.to;

        for x in x0..=x1 {
            for y in y0..=y1 {
                let light = &mut self.lights[x + y * 1000];
                *light = match self.mode {
                    Mode::P1 => op.exec(*light),
                    Mode::P2 => op.exec_p2(*light),
                };
            }
        }
    }
}

fn skip_cmd(s: &str) -> (OpType, Skip<Chars<'_>>) {
    let chars = s.chars();
    for (cmd, op_type) in [
        (CMD_TURN_ON, OpType::On),
        (CMD_TURN_OFF, OpType::Off),
        (CMD_TOGGLE, OpType::Toggle),
    ] {
        if s.starts_with(cmd) {
            return (op_type, chars.skip(cmd.len() + 1));
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_parse() {
        assert_eq!(
            Op::parse("turn on 0,0 through 999,999"),
            Op::new(OpType::On, (0, 0), (999, 999))
        );
        assert_eq!(
            Op::parse("toggle 0,0 through 999,0"),
            Op::new(OpType::Toggle, (0, 0), (999, 0))
        );
        assert_eq!(
            Op::parse("turn off 499,499 through 500,500"),
            Op::new(OpType::Off, (499, 499), (500, 500))
        );
    }
    #[test]
    fn test_p1() {
        assert_eq!(
            p1("turn on 0,0 through 999,999\n\
            toggle 0,0 through 999,0\n\
            turn off 499,499 through 500,500"),
            998996
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("turn on 0,0 through 0,0"), 1);
        assert_eq!(p2("toggle 0,0 through 999,999"), 2_000_000);
    }
}
