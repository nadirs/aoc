use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    sequence::{preceded, terminated},
    IResult,
};

pub fn p1(input: &str) -> i64 {
    best_recipe(input, |score, new_score, _| new_score > score)
}

pub fn p2(input: &str) -> i64 {
    best_recipe(input, |score, new_score, cals| {
        cals == 500 && new_score > score
    })
}

fn best_recipe(input: &str, predicate: fn(i64, i64, i64) -> bool) -> i64 {
    let ingredients = parse_input(input);
    let mut score = 0;
    let max_stars = 100;
    let max_sticks = ingredients.len();
    let mut track = VecDeque::from([(max_stars, max_sticks, Vec::new())]);

    while let Some((stars, sticks, weights)) = track.pop_front() {
        if weights.len() < max_sticks {
            for i in 0..=stars {
                track.push_back((
                    stars - i,
                    sticks - 1,
                    weights.iter().chain([&i]).cloned().collect(),
                ))
            }
        } else {
            let (new_score, cals) = get_score(&ingredients, &weights);
            if predicate(score, new_score, cals) {
                score = new_score;
            }
        }
    }

    score
}

type Ingredient = (i64, i64, i64, i64, i64);

fn get_score(ingredients: &[Ingredient], weights: &[i64]) -> (i64, i64) {
    let (score, cals) = ingredients
        .iter()
        .zip(weights)
        .map(|((cap, dur, fla, tex, cal), w)| (cap * w, dur * w, fla * w, tex * w, cal * w))
        .fold(([0, 0, 0, 0], 0), |(acc, cal), ingr| {
            (
                [
                    acc[0] + ingr.0,
                    acc[1] + ingr.1,
                    acc[2] + ingr.2,
                    acc[3] + ingr.3,
                ],
                cal + ingr.4,
            )
        });

    (score.map(|n| n.max(0)).iter().product(), cals)
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input
        .trim()
        .lines()
        .map(|l| parse_ingredient(l).unwrap().1)
        .collect()
}

fn parse_ingredient(input: &str) -> IResult<&str, Ingredient> {
    let (input, _) = terminated(alpha1, tag(": "))(input)?;
    let (input, capacity) = preceded(tag("capacity "), i64)(input)?;
    let (input, durability) = preceded(tag(", durability "), i64)(input)?;
    let (input, flavor) = preceded(tag(", flavor "), i64)(input)?;
    let (input, texture) = preceded(tag(", texture "), i64)(input)?;
    let (input, calories) = preceded(tag(", calories "), i64)(input)?;

    Ok((input, (capacity, durability, flavor, texture, calories)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_p1() {
        assert_eq!(
            parse_input(INPUT),
            vec![(-1, -2, 6, 3, 8), (2, 3, -2, -1, 3)]
        );

        assert_eq!(get_score(&parse_input(INPUT), &[44, 56]).0, 62842880);
        assert_eq!(p1(INPUT), 62842880);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 57600000);
    }
}
