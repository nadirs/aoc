use std::collections::VecDeque;

pub fn p1(input: &str) -> usize {
    p1_times(input, 40)
}

fn p1_times(input: &str, mut times: usize) -> usize {
    let mut s = input.trim().to_string();
    while times > 0 {
        times -= 1;
        s = look_and_say(&s);
    }

    s.len()
}

pub fn p2(input: &str) -> usize {
    p1_times(input, 50)
}

fn look_and_say(input: &str) -> String {
    let groups = input
        .chars()
        .into_iter()
        .fold(VecDeque::new(), |mut acc, d| {
            if let Some((d0, n)) = acc.back_mut() {
                if *d0 == d {
                    *n += 1;
                } else {
                    acc.push_back((d, 1));
                }
            } else {
                acc.push_back((d, 1u8))
            }

            acc
        });

    groups
        .into_iter()
        .flat_map(|(d, n)| [(b'0' + n) as char, d])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");

        assert_eq!(p1_times("1", 5), "312211".len());
        assert_eq!(look_and_say("1113222113"), "3113322113");
    }
}
