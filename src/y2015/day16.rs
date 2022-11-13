use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

type Compounds = BTreeMap<String, u64>;

fn get_detection() -> Compounds {
    Compounds::from([
        ("children".into(), 3),
        ("cats".into(), 7),
        ("samoyeds".into(), 2),
        ("pomeranians".into(), 3),
        ("akitas".into(), 0),
        ("vizslas".into(), 0),
        ("goldfish".into(), 5),
        ("trees".into(), 3),
        ("cars".into(), 2),
        ("perfumes".into(), 1),
    ])
}

pub fn p1(input: &str) -> u64 {
    find_aunt_by(input, |detection, k, v| detection[k] == v)
}

pub fn p2(input: &str) -> u64 {
    find_aunt_by(input, |detection, k, v| {
        if k == "cats" || k == "trees" {
            detection[k] < v
        } else if k == "pomeranians" || k == "goldfish" {
            detection[k] > v
        } else {
            detection[k] == v
        }
    })
}

fn find_aunt_by(input: &str, predicate: fn(&Compounds, &str, u64) -> bool) -> u64 {
    let detection = get_detection();
    let aunts = parse_aunts(input);

    aunts
        .into_iter()
        .find(|(_, aunt)| aunt.iter().all(|(k, v)| predicate(&detection, k, *v)))
        .unwrap()
        .0
}

fn parse_aunts(input: &str) -> impl Iterator<Item = (u64, Compounds)> + '_ {
    input.trim().lines().map(|l| parse_aunt(l).unwrap().1)
}

fn parse_aunt(input: &str) -> IResult<&str, (u64, Compounds)> {
    let (input, index) = delimited(tag("Sue "), u64, tag(": "))(input)?;
    let (input, compounds) = separated_list1(tag(", "), parse_compound)(input)?;

    Ok((input, (index, Compounds::from_iter(compounds))))
}

fn parse_compound(input: &str) -> IResult<&str, (String, u64)> {
    separated_pair(map(alpha1, String::from), tag(": "), u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(
            parse_aunt("Sue 123: children: 1, cars: 8, vizslas: 7")
                .unwrap()
                .1,
            (
                123,
                Compounds::from([
                    ("children".into(), 1),
                    ("cars".into(), 8),
                    ("vizslas".into(), 7),
                ])
            )
        );
    }
}
