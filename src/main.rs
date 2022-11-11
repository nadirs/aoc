pub mod y2015;

fn main() -> Result<(), aoc::Error> {
    let input = aoc::pull_input(2015, 8)?;

    println!("{:?}", y2015::day08::p1(&input));
    println!("{:?}", y2015::day08::p2(&input));

    Ok(())
}
