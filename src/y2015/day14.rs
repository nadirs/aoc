use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

pub fn p1(input: &str) -> usize {
    run_race(input, 2503)
}

pub fn p2(input: &str) -> usize {
    run_race_p2(input, 2503)
}

type Name = String;
type Spec = (usize, usize, usize);

fn run_race(input: &str, time: usize) -> usize {
    parsed_specs(input)
        .map(|spec| compute_distance(spec, time))
        .max()
        .unwrap()
}

fn run_race_p2(input: &str, time: usize) -> usize {
    let specs: Vec<_> = parsed_specs(input).collect();

    let mut scores = vec![0; specs.len()];

    for t in 1..=time {
        let scored_specs: Vec<_> = specs
            .iter()
            .map(|spec| compute_distance(*spec, t))
            .enumerate()
            .collect();
        let (_, best_score) = scored_specs.iter().max_by_key(|(_, score)| *score).unwrap();

        for (i, score) in &scored_specs {
            if score == best_score {
                scores[*i] += 1;
            }
        }
    }

    scores.into_iter().max().unwrap()
}

fn parsed_specs(input: &str) -> impl Iterator<Item = Spec> + '_ {
    input.trim().lines().map(|l| parse_line(l).unwrap().1 .1)
}

fn compute_distance((speed, run_time, rest_time): Spec, time: usize) -> usize {
    let full_runs = run_time * (time / (run_time + rest_time));
    let partial_run = (time % (run_time + rest_time)).min(run_time);

    speed * (full_runs + partial_run)
}

fn parse_line(input: &str) -> IResult<&str, (Name, Spec)> {
    tuple((parse_name, parse_spec))(input)
}

fn parse_name(input: &str) -> IResult<&str, Name> {
    map(alpha1, Name::from)(input)
}

fn parse_spec(input: &str) -> IResult<&str, Spec> {
    tuple((
        preceded(tag(" can fly "), parse_num),
        preceded(tag(" km/s for "), parse_num),
        preceded(tag(" seconds, but then must rest for "), parse_num),
    ))(input)
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

    #[test]
    fn test_p1() {
        assert_eq!(
            parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
            Ok((" seconds.", ("Comet".into(), (14, 10, 127))))
        );
        assert_eq!(
            parse_line(
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            ),
            Ok((" seconds.", ("Dancer".into(), (16, 11, 162))))
        );

        assert_eq!(compute_distance((14, 10, 127), 1000), 1120);
        assert_eq!(compute_distance((14, 10, 127), 1), 14);
        assert_eq!(compute_distance((14, 10, 127), 2), 28);
        assert_eq!(compute_distance((14, 10, 127), 10), 140);
        assert_eq!(compute_distance((14, 10, 127), 11), 140);
        assert_eq!(compute_distance((14, 10, 127), 137), 140);
        assert_eq!(compute_distance((14, 10, 127), 138), 154);

        assert_eq!(compute_distance((16, 11, 162), 1000), 1056);
        assert_eq!(run_race(INPUT, 1000), 1120);
    }

    #[test]
    fn test_p2() {
        assert_eq!(run_race_p2(INPUT, 1000), 689);
    }
}
