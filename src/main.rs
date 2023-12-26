pub mod y2023;

fn main() -> Result<(), aoc::Error> {
    let input = aoc::pull_input(2023, 5)?;

    println!("{}", y2023::day05::p1(&input));
    println!("{}", y2023::day05::p2(&input));

    Ok(())
}
