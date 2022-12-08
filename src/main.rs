pub mod y2022;

fn main() -> Result<(), aoc::Error> {
    let input = aoc::pull_input(2022, 6)?;

    println!("{}", y2022::day06::p1(&input));
    println!("{}", y2022::day06::p2(&input));

    Ok(())
}
