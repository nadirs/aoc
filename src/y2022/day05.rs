use std::collections::VecDeque;

pub fn p1(input: &str) -> String {
    let (drawing, steps) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_drawing(drawing);

    for (mut column, source, dest) in parse_steps(steps) {
        while column > 0 {
            let c = stacks[source - 1].pop_front().unwrap();
            stacks[dest - 1].push_front(c);
            column -= 1;
        }
    }

    stacks.iter().map(|col| col[0]).collect()
}

pub fn p2(input: &str) -> String {
    let (drawing, steps) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_drawing(drawing);

    for (mut column, source, dest) in parse_steps(steps) {
        let mut acc = VecDeque::with_capacity(column);
        while column > 0 {
            let c = stacks[source - 1].pop_front().unwrap();
            acc.push_back(c);
            column -= 1;
        }
        stacks[dest - 1].extend(acc.iter());
        stacks[dest - 1].rotate_right(acc.len());
    }

    stacks.iter().map(|col| col[0]).collect()
}

type Step = (usize, usize, usize);

fn parse_drawing(drawing: &str) -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();
    let drawing_height = drawing.lines().count() - 1;

    for (_, line) in drawing
        .lines()
        .enumerate()
        .take_while(|(i, _)| *i < drawing_height)
    {
        let mut peek = line.chars();
        let mut stack_index = 0;

        while peek.next().is_some() {
            if stack_index >= stacks.len() {
                stacks.push(VecDeque::new());
            }
            if let Some(c) = peek.next() {
                if c.is_alphabetic() {
                    stacks[stack_index].push_back(c);
                }
            }
            peek.next();
            peek.next();

            stack_index += 1;
        }
    }

    stacks
}

fn parse_steps(input: &str) -> impl Iterator<Item = Step> + '_ {
    input.lines().map(|l| {
        let (_move, rest) = l.split_once(' ').unwrap();
        let (col, rest) = rest.split_once(' ').unwrap();
        let (_from, rest) = rest.split_once(' ').unwrap();
        let (source, rest) = rest.split_once(' ').unwrap();
        let (_to, dest) = rest.split_once(' ').unwrap();

        (
            col.parse().unwrap(),
            source.parse().unwrap(),
            dest.parse().unwrap(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_drawing() {
        let drawing = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
";
        assert_eq!(
            parse_drawing(drawing),
            vec![
                VecDeque::from(['N', 'Z']),
                VecDeque::from(['D', 'C', 'M']),
                VecDeque::from(['P']),
            ]
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), "CMZ");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), "MCD");
    }
}
