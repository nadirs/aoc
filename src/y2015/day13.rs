use std::collections::{BTreeMap, BTreeSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    combinator::map_res,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

type Name = String;
type Happiness = isize;
type Graph = BTreeMap<Name, BTreeMap<Name, Happiness>>;

pub fn p1(input: &str) -> isize {
    let graph = parse_graph(input);
    happiest_path(graph)
}

pub fn p2(input: &str) -> isize {
    let mut graph = parse_graph(input);
    let myself = "Myself";

    for edges in graph.values_mut() {
        edges.insert(myself.into(), 0);
    }

    graph.insert(myself.into(), graph.keys().map(|k| (k.into(), 0)).collect());

    happiest_path(graph)
}

fn happiest_path(graph: Graph) -> isize {
    let mut result = isize::MIN;

    let path_len = graph.len();

    let start = graph.iter().next().unwrap().0;
    let mut track = VecDeque::from([(vec![start], BTreeSet::from([start]), 0)]);

    while let Some((path, visits, mut happiness)) = track.pop_front() {
        let &node = path.last().unwrap();

        if path.len() == path_len {
            let &start = path.first().unwrap();
            happiness += graph.get(start).unwrap().get(node).unwrap()
                + graph.get(node).unwrap().get(start).unwrap();
            if happiness > result {
                result = happiness;
                continue;
            }
        }

        let edges = graph.get(node).unwrap();

        for (edge_name, edge_cost) in edges {
            if visits.contains(edge_name) {
                continue;
            }

            let reversed_cost = graph.get(edge_name).unwrap().get(node).unwrap();

            track.push_back((
                path.iter().chain([&edge_name]).cloned().collect(),
                visits.iter().chain([&edge_name]).cloned().collect(),
                happiness + edge_cost + reversed_cost,
            ));
        }
    }

    result
}

fn parse_graph(input: &str) -> Graph {
    input
        .trim()
        .lines()
        .fold(Default::default(), |mut graph, line| {
            let (a, b, h) = parse_line(line).unwrap().1;
            graph.entry(a).or_default().insert(b, h);
            graph
        })
}

fn parse_line(input: &str) -> IResult<&str, (Name, Name, Happiness)> {
    let (input, a) = terminated(parse_name, tag(" "))(input)?;
    let (input, (h, b)) = separated_pair(
        parse_happiness,
        tag(" happiness units by sitting next to "),
        parse_name,
    )(input)?;

    Ok((input, (a, b, h)))
}

fn parse_name(input: &str) -> IResult<&str, Name> {
    map(alpha1, String::from)(input)
}

fn parse_happiness(input: &str) -> IResult<&str, Happiness> {
    let parse_num = map_res(digit1, |n: &str| n.parse::<Happiness>());
    let parse_gain = map(tag("would gain "), |_| 1);
    let parse_lose = map(tag("would lose "), |_| -1);

    map(
        tuple((alt((parse_gain, parse_lose)), parse_num)),
        |(sign, num)| sign * num,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_p1() {
        assert_eq!(parse_graph(INPUT), expected_graph());
        assert_eq!(p1(INPUT), 330);
    }

    fn expected_graph() -> Graph {
        BTreeMap::from([
            (
                "Alice".into(),
                BTreeMap::from([
                    ("Bob".into(), 54),
                    ("Carol".into(), -79),
                    ("David".into(), -2),
                ]),
            ),
            (
                "Bob".into(),
                BTreeMap::from([
                    ("Alice".into(), 83),
                    ("Carol".into(), -7),
                    ("David".into(), -63),
                ]),
            ),
            (
                "Carol".into(),
                BTreeMap::from([
                    ("Alice".into(), -62),
                    ("Bob".into(), 60),
                    ("David".into(), 55),
                ]),
            ),
            (
                "David".into(),
                BTreeMap::from([
                    ("Alice".into(), 46),
                    ("Bob".into(), -7),
                    ("Carol".into(), 41),
                ]),
            ),
        ])
    }
}
