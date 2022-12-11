use std::iter::repeat;

pub fn p1(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut count = (grid.h + grid.w - 2) * 2;
    for (x, y) in grid.iter() {
        if grid.can_see(x, y) {
            count += 1;
        }
    }

    count
}

pub fn p2(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut score = 0;
    for (x, y) in grid.iter() {
        score = score.max(grid.scenic_score(x, y));
    }

    score
}

struct Grid {
    data: Vec<usize>,
    h: usize,
    w: usize,
}

impl Grid {
    fn at(&self, x: usize, y: usize) -> usize {
        self.data[x + y * self.w]
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (1..(self.h - 1)).flat_map(|y| (1..(self.w - 1)).zip(repeat(y)))
    }

    fn can_see(&self, x: usize, y: usize) -> bool {
        let n = self.at(x, y);

        (0..y).all(|i| self.at(x, i) < n)
            || ((y + 1)..self.h).all(|i| self.at(x, i) < n)
            || (0..x).all(|i| self.at(i, y) < n)
            || ((x + 1)..self.w).all(|i| self.at(i, y) < n)
    }

    fn range_score(&self, base: usize, coords: impl Iterator<Item = (usize, usize)>) -> usize {
        let mut score = 0;
        for (dx, dy) in coords {
            score += 1;
            if self.at(dx, dy) >= base {
                break;
            }
        }
        score
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let base = self.at(x, y);

        self.range_score(base, (0..x).rev().zip(repeat(y)))
            * self.range_score(base, ((x + 1)..self.w).zip(repeat(y)))
            * self.range_score(base, (repeat(x)).zip((0..y).rev()))
            * self.range_score(base, (repeat(x)).zip((y + 1)..self.h))
    }
}

fn parse_grid(input: &str) -> Grid {
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();

    let mut data = Vec::with_capacity(h * w);

    for l in input.lines() {
        for c in l.chars() {
            data.push(c.to_digit(10).unwrap() as usize);
        }
    }

    Grid { data, h, w }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 8);
    }
}
