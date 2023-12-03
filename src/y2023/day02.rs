#[derive(Debug, Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubes {
    fn can_contain(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

pub fn p1(input: &str) -> usize {
    let spec = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    parse_input(input)
        .filter_map(|(game_id, cubes)| {
            if spec.can_contain(&cubes) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    parse_input(input).map(|(_, cubes)| cubes.power()).sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, Cubes)> + '_ {
    input.trim().lines().map(|l| {
        let (head, sets) = l.split_once(": ").unwrap();
        let (_, game_id) = head.split_once(" ").unwrap();
        let cubes = sets.split("; ").fold(Cubes::default(), |mut acc, x| {
            for (num, color) in x.split(", ").map(|c| c.split_once(' ').unwrap()) {
                let num = num.parse::<usize>().unwrap();
                match color {
                    "red" => acc.red = acc.red.max(num),
                    "green" => acc.green = acc.green.max(num),
                    "blue" => acc.blue = acc.blue.max(num),
                    _ => unreachable!("unexpected color {color}"),
                }
            }
            acc
        });
        (game_id.parse::<usize>().unwrap(), cubes)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 8);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 2286);
    }
}
