pub mod y2022;

fn main() -> Result<(), aoc::Error> {
    let input = aoc::pull_input(2022, 8)?;

    println!("{}", y2022::day08::p1(&input));
    println!("{}", y2022::day08::p2(&input));

    Ok(())
}
