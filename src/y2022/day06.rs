use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    solve(input, 4)
}

pub fn p2(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, length: usize) -> usize {
    let mut i = 0;
    while i < input.len() - length {
        if input[i..(i + length)].chars().collect::<HashSet<_>>().len() == length {
            break;
        }
        i += 1;
    }
    i + length
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: &[&str] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_p1() {
        let solutions = &[7, 5, 6, 10, 11];
        for (&input, &solution) in INPUTS.iter().zip(solutions) {
            assert_eq!(p1(input), solution);
        }
    }

    #[test]
    fn test_p2() {
        let solutions = &[19, 23, 23, 29, 26];
        for (&input, &solution) in INPUTS.iter().zip(solutions) {
            assert_eq!(p2(input), solution);
        }
    }
}
