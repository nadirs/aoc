use std::{collections::HashMap, ops::Range};

pub fn p1(input: &str) -> usize {
    let schema = Schema::parse(input);

    schema
        .get_num_coords()
        .into_iter()
        .filter(|coord| {
            schema
                .get_neighbors(coord)
                .any(|(ny, nx)| is_symbol(schema.data[ny][nx]))
        })
        .map(|coords| schema.parse_number(&coords))
        .sum()
}

type GearMap = HashMap<(usize, usize), Vec<Coords>>;

pub fn p2(input: &str) -> usize {
    let schema = Schema::parse(input);

    schema
        .get_num_coords()
        .into_iter()
        .fold(GearMap::new(), |mut acc, coord| {
            for neighbor in schema
                .get_neighbors(&coord)
                .filter(|&(ny, nx)| schema.data[ny][nx] == '*')
            {
                acc.entry(neighbor).or_default().push(coord.clone());
            }

            acc
        })
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, coords)| schema.parse_number(&coords[0]) * schema.parse_number(&coords[1]))
        .sum()
}

type Coords<T = usize> = (T, Range<T>);

struct Schema {
    data: Vec<Vec<char>>,
}

impl Schema {
    fn parse(input: &str) -> Self {
        Self {
            data: input.trim().lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn parse_number(&self, (y, xrange): &Coords) -> usize {
        self.data[*y][xrange.start..xrange.end]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .expect("Unable to parse number")
    }

    fn get_num_coords(&self) -> Vec<(usize, Range<usize>)> {
        let mut coords: Vec<Coords> = Vec::new();
        for (y, row) in self.data.iter().enumerate() {
            let mut currently_digit = false;

            for x in 0..row.len() {
                if is_digit(row[x]) {
                    if !currently_digit {
                        coords.push((y, x..row.len()));
                        currently_digit = true;
                    }
                } else {
                    if currently_digit {
                        currently_digit = false;
                        let coord = coords.last_mut().expect("Coords can't be empty");
                        coord.1.end = x;
                    }
                }
            }
        }
        coords
    }

    fn get_neighbors(&self, (y, xrange): &Coords) -> impl Iterator<Item = (usize, usize)> {
        let mut result = Vec::with_capacity(xrange.len() * 2 + 6);

        let x0 = if xrange.start > 0 {
            xrange.start - 1
        } else {
            xrange.start
        };
        // xrange.end is exclusive, just like row_len
        let x1 = if xrange.end < self.row_len() {
            xrange.end + 1
        } else {
            xrange.end
        };

        // top
        if *y > 0 {
            for x in x0..x1 {
                result.push((y - 1, x));
            }
        }
        // left
        if xrange.start > 0 {
            result.push((*y, xrange.start - 1));
        }
        // right
        if xrange.end < self.row_len() {
            result.push((*y, xrange.end));
        }
        // bottom
        if *y + 1 < self.data.len() {
            for x in x0..x1 {
                result.push((y + 1, x));
            }
        }

        result.into_iter()
    }

    fn row_len(&self) -> usize {
        self.data.first().map(|row| row.len()).unwrap_or(0)
    }
}

fn is_symbol(cell: char) -> bool {
    cell != '.' && !is_digit(cell)
}

fn is_digit(cell: char) -> bool {
    cell >= '0' && cell <= '9'
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*...2
.664.598..";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 4361);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 467835);
    }

    #[test]
    fn test_is_symbol() {
        assert!(!is_symbol('.'));
        assert!(!is_symbol('0'));
        assert!(!is_symbol('1'));
        assert!(!is_symbol('8'));
        assert!(!is_symbol('9'));

        assert!(is_symbol('#'));
        assert!(is_symbol('$'));
        assert!(is_symbol('+'));
        assert!(is_symbol('*'));
    }
}

aoc::solve!(2023, 3, p1, p2);
