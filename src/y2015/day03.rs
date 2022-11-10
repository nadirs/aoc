use std::collections::HashMap;

pub fn p1(input: &str) -> usize {
    let mut santa = Santa::default();
    let mut map = HashMap::new();

    for c in input.chars() {
        santa.go(c, &mut map);
    }

    map.len()
}

pub fn p2(input: &str) -> usize {
    let santa = Santa::default();
    let robot = Santa::default();
    let mut map = HashMap::new();

    let mut santas = [santa, robot];

    let mut i = 0;
    for c in input.chars() {
        i = (i + 1) % 2;
        santas[i].go(c, &mut map);
    }

    map.len()
}

type Pos = (isize, isize);
type Map = HashMap<Pos, usize>;

#[derive(Default)]
struct Santa {
    pos: Pos,
}

impl Santa {
    fn go(&mut self, c: char, map: &mut Map) {
        let next_pos = self.next_pos(c);

        self.visit(map);
        self.pos = next_pos;
        self.visit(map);
    }

    fn visit(&self, map: &mut Map) {
        map.entry(self.pos).and_modify(|p| *p += 1).or_insert(1);
    }

    fn offset(c: char) -> Pos {
        match c {
            '>' => (1, 0),
            '^' => (0, -1),
            '<' => (-1, 0),
            'v' => (0, 1),
            _ => unreachable!(),
        }
    }

    fn next_pos(&self, c: char) -> Pos {
        let (x, y) = self.pos;
        let (dx, dy) = Self::offset(c);

        (x + dx, y + dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(">"), 2);
        assert_eq!(p1("^>v<"), 4);
        assert_eq!(p1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("^>v<"), 3);
        assert_eq!(p2("^v^v^v^v^v"), 11);
    }
}
