use std::collections::{HashMap, HashSet, VecDeque};

pub fn p1(input: &str) -> usize {
    let (routes, locset) = parse_routes(input);
    shortest_path(&routes, &locset)
}

pub fn p2(input: &str) -> usize {
    let (routes, locset) = parse_routes(input);
    longest_path(&routes, &locset)
}

type Loc<'a> = &'a str;
type Dist = usize;
type RouteMap<'a> = HashMap<Loc<'a>, HashMap<Loc<'a>, Dist>>;
type LocSet<'a> = HashSet<Loc<'a>>;

fn parse_routes(input: &str) -> (RouteMap<'_>, LocSet<'_>) {
    let mut map = RouteMap::default();
    let mut locs = LocSet::default();

    for l in input.lines() {
        let (loc1, rest) = l.split_once(" to ").unwrap();
        let (loc2, rest) = rest.split_once(" = ").unwrap();
        let dist = usize::from_str_radix(rest, 10).unwrap();

        add_route(&mut map, &mut locs, loc1, loc2, dist);
        add_route(&mut map, &mut locs, loc2, loc1, dist);
    }

    (map, locs)
}

fn add_route<'a>(
    map: &mut RouteMap<'a>,
    locs: &mut LocSet<'a>,
    src: Loc<'a>,
    dst: Loc<'a>,
    dist: usize,
) {
    locs.insert(src);
    map.entry(src).or_default().insert(dst, dist);
}

fn shortest_path(routes: &RouteMap<'_>, locset: &LocSet<'_>) -> usize {
    fold_path(routes, locset, usize::MAX, |acc, dist| dist < acc)
}

fn longest_path(routes: &RouteMap<'_>, locset: &LocSet<'_>) -> usize {
    fold_path(routes, locset, 0, |acc, dist| dist > acc)
}

fn fold_path(
    routes: &RouteMap<'_>,
    locset: &LocSet<'_>,
    mut acc: usize,
    predicate: fn(usize, usize) -> bool,
) -> usize {
    let mut best_path = vec![];
    let path_len = locset.len();

    // any starting point is possible
    for &start in locset {
        let path = vec![start];
        let mut track = VecDeque::from([(path, LocSet::from([start]), 0)]);

        while let Some((path, visits, total_dist)) = track.pop_front() {
            // cheap way to check that we've visited every location
            if path.len() == path_len && predicate(acc, total_dist) {
                acc = total_dist;
                best_path = path;
                continue;
            }

            let node = path.last().unwrap();
            let edges = routes.get(*node).unwrap();

            // breadth first
            for (edge, weight) in edges {
                if visits.contains(edge) {
                    continue;
                }
                track.push_back((
                    path.iter().chain([edge]).cloned().collect(),
                    visits.iter().chain([edge]).cloned().collect(),
                    total_dist + weight,
                ));
            }
        }
    }

    println!("{}", best_path.join(" -> "));
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "London to Dublin = 464\n\
        London to Belfast = 518\n\
        Dublin to Belfast = 141";

        let expected_map: RouteMap = RouteMap::from([
            ("London", HashMap::from([("Dublin", 464), ("Belfast", 518)])),
            ("Dublin", HashMap::from([("Belfast", 141), ("London", 464)])),
            ("Belfast", HashMap::from([("London", 518), ("Dublin", 141)])),
        ]);
        let expected_locs: LocSet = LocSet::from(["London", "Dublin", "Belfast"]);

        let (routes, locs) = parse_routes(input);
        assert_eq!(routes, expected_map);
        assert_eq!(locs, expected_locs);

        assert_eq!(shortest_path(&routes, &locs), 605);
    }

    #[test]
    fn test_p2() {
        let input = "London to Dublin = 464\n\
        London to Belfast = 518\n\
        Dublin to Belfast = 141";

        assert_eq!(p2(input), 982);
    }
}
