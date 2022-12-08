use std::collections::HashMap;

pub fn p1(input: &str) -> usize {
    parse_dirs(input)
        .into_values()
        .filter(|&v| v <= 100000)
        .sum()
}

pub fn p2(input: &str) -> usize {
    let dirs = parse_dirs(input);
    let occupied_size = dirs.values().max().unwrap();
    let required_space = 30_000_000 - (70_000_000 - occupied_size);

    dirs.into_values()
        .filter(|&v| v >= required_space)
        .min()
        .unwrap()
}

fn parse_dirs(input: &str) -> HashMap<Vec<&str>, usize> {
    let mut dirs = Vec::new();
    let mut sizes = HashMap::new();

    for l in input.trim().lines() {
        if l == "$ cd .." {
            dirs.pop();
        } else if l.starts_with("$ cd") {
            dirs.push(l);
            sizes.insert(dirs.clone(), 0);
        } else if l.starts_with("$ ls") || l.starts_with("dir") {
            continue;
        } else {
            let (size, _) = l.split_once(' ').unwrap();
            let mut redirs = dirs.clone();
            loop {
                sizes
                    .entry(redirs.clone())
                    .and_modify(|v| *v += size.parse::<usize>().unwrap());
                if redirs.pop().is_none() {
                    break;
                }
            }
        }
    }

    sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 95437);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 24933642);
    }
}
