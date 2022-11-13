use std::collections::HashMap;

pub fn p1(input: &str) -> usize {
    let tot = 150;
    let containers = parse_input(input);

    let (solution, _) = resolve(tot, &containers);

    solution
}

pub fn p2(input: &str) -> usize {
    let tot = 150;
    let containers = parse_input(input);

    let (_, solution) = resolve(tot, &containers);

    solution
}

type Solutions = HashMap<u64, usize>;

fn resolve(tot: u64, containers: &[u64]) -> (usize, usize) {
    let mut sol_p1 = 0;
    let mut solutions = Solutions::new();

    let mut track = vec![(tot, 0, 0)];

    while let Some((leftover, current, buckets)) = track.pop() {
        for (i, &capacity) in containers.iter().skip(current).enumerate() {
            if leftover == capacity {
                solutions
                    .entry(buckets + 1)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                sol_p1 += 1;
                continue;
            }

            if leftover < capacity {
                continue;
            }

            track.push((leftover - capacity, current + i + 1, buckets + 1));
        }
    }

    let min = solutions.keys().min().unwrap();
    let sol_p2 = solutions[min];

    (sol_p1, sol_p2)
}

fn parse_input(input: &str) -> Vec<u64> {
    input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(resolve(25, &[20, 15, 10, 5, 5]).0, 4);
    }

    #[test]
    fn test_p2() {
        assert_eq!(resolve(25, &[20, 15, 10, 5, 5]).1, 3);
    }
}
