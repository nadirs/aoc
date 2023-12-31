use std::collections::BTreeMap;

pub fn p1(input: &str) -> usize {
    let (directions, map) = parse_input(input);
    count_steps(directions, "AAA", &map, |x| x == "ZZZ")
}

pub fn p2(input: &str) -> usize {
    let (directions, map) = parse_input(input);

    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| count_steps(directions, start, &map, |x| x.ends_with('Z')))
        .reduce(lcm)
        .unwrap()
}

fn count_steps(
    directions: &str,
    start: &str,
    map: &BTreeMap<&str, (&str, &str)>,
    is_end: fn(&str) -> bool,
) -> usize {
    let mut step_count = 0;
    let mut cur = start;
    for dir in directions.chars().cycle() {
        cur = match dir {
            'L' => map[cur].0,
            'R' => map[cur].1,
            dir => unreachable!("Invalid dir {dir}"),
        };
        step_count += 1;
        if is_end(cur) {
            break;
        }
    }

    step_count
}

fn parse_input(input: &str) -> (&str, BTreeMap<&str, (&str, &str)>) {
    let (directions, rest) = input.trim().split_once("\n\n").unwrap();

    let map: BTreeMap<_, _> = rest
        .lines()
        .map(|line| {
            let (k, branches) = line.split_once(" = ").unwrap();
            let v = branches.trim_matches(['(', ')']).split_once(", ").unwrap();
            (k, v)
        })
        .collect();
    (directions, map)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT_ALT: &str = "LLR

BBB = (AAA, ZZZ)
AAA = (BBB, BBB)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 2);
        assert_eq!(p1(INPUT_ALT), 6);
    }

    #[test]
    fn test_p2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(p2(input), 6);
    }
}

aoc::solve!(2023, 8, p1, p2);
