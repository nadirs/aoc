pub fn p1(input: &str) -> usize {
    solve(input, |left, right| {
        let (them, me) = (left.into(), right.into());

        (me, Outcome::from((them, me)))
    })
}

pub fn p2(input: &str) -> usize {
    solve(input, |left, right| {
        let (them, outcome): (Choice, Outcome) = (left.into(), right.into());

        (outcome.vs(them), outcome)
    })
}

fn solve(input: &str, f: fn(char, char) -> (Choice, Outcome)) -> usize {
    input
        .lines()
        .map(|l| {
            let (me, outcome) = f(l.chars().next().unwrap(), l.chars().nth(2).unwrap());
            me.score() + outcome.score()
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn beats(self) -> Self {
        match self {
            Self::Rock => Self::Scissor,
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
        }
    }
}

impl From<char> for Choice {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
    fn vs(self, choice: Choice) -> Choice {
        match self {
            Self::Lose => choice.beats(),
            Self::Draw => choice,
            Self::Win => choice.beats().beats(),
        }
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl From<(Choice, Choice)> for Outcome {
    fn from((them, me): (Choice, Choice)) -> Self {
        if them == me {
            Self::Draw
        } else if them.beats() == me {
            Self::Lose
        } else {
            Self::Win
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 15);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 12);
    }
}
