pub mod y2015;

fn main() -> Result<(), aoc::Error> {
    let input = aoc::pull_input(2015, 14)?;

    println!("{}", y2015::day14::p1(&input));
    println!("{}", y2015::day14::p2(&input));

    Ok(())
}
