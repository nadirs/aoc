pub mod y2022;

fn main() -> Result<(), aoc::Error> {
    let input = aoc::pull_input(2022, 5)?;

    println!("{}", y2022::day05::p1(&input));
    println!("{}", y2022::day05::p2(&input));

    Ok(())
}
