use std::{ops::Range, str::Lines};

pub fn p1(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let seeds = parse_nums(lines.next().unwrap().split_once(' ').unwrap().1);
    let maps = parse_maps(lines);

    seeds
        .map(|seed| maps.iter().fold(seed, |acc, map| remap(acc, map)))
        .min()
        .unwrap()
}

pub fn p2(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let seeds: Vec<_> = parse_seeds(&mut lines).collect();
    let seed_pairs = seeds
        .iter()
        .zip(seeds.iter().skip(1))
        .enumerate()
        .filter_map(|(i, pair)| if i % 2 == 0 { Some(pair) } else { None });
    let maps = parse_maps(lines);

    seed_pairs
        .flat_map(|(&start, &len)| {
            let end = start + len;
            (start..end).map(|seed| maps.iter().fold(seed, |acc, map| remap(acc, map)))
        })
        .min()
        .unwrap()
}

fn parse_seeds<'a>(lines: &mut Lines<'a>) -> impl Iterator<Item = usize> + 'a {
    parse_nums(lines.next().unwrap().split_once(' ').unwrap().1)
}

fn parse_nums(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split(' ').map(|s| s.parse::<usize>().unwrap())
}

type RangePair = (Range<usize>, Range<usize>);

fn parse_maps(lines: Lines) -> Vec<Vec<RangePair>> {
    let (seed_map, lines) = parse_map(lines);
    let (soil_map, lines) = parse_map(lines);
    let (fertilizer_map, lines) = parse_map(lines);
    let (water_map, lines) = parse_map(lines);
    let (light_map, lines) = parse_map(lines);
    let (temperature_map, lines) = parse_map(lines);
    let (humidity_map, lines) = parse_map(lines);
    let (location_map, _) = parse_map(lines);

    vec![
        seed_map,
        soil_map,
        fertilizer_map,
        water_map,
        light_map,
        temperature_map,
        humidity_map,
        location_map,
    ]
}

fn parse_map<'a, Input: Iterator<Item = &'a str>>(
    lines: Input,
) -> (Vec<RangePair>, impl Iterator<Item = &'a str>) {
    let mut rest = lines.skip_while(|l| l.trim().is_empty() || l.contains("map"));
    let mut result = Vec::new();
    while let Some(l) = rest.next() {
        if l.trim().is_empty() {
            break;
        }

        let mut nums = parse_nums(l);
        let dest = nums.next().unwrap();
        let src = nums.next().unwrap();
        let len = nums.next().unwrap();
        result.push((dest..(dest + len), src..(src + len)));
    }

    (result, rest)
}

fn remap(value: usize, map: &[RangePair]) -> usize {
    for entry in map {
        if entry.1.contains(&value) {
            let offset = value - entry.1.start;
            return entry.0.start + offset;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 35);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 46);
    }
}

aoc::solve!(2023, 5, p1, p2);
