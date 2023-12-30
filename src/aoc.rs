use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Error(String);
impl From<String> for Error {
    fn from(e: String) -> Self {
        Self(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self(e.to_string())
    }
}

pub fn pull_input(y: usize, d: usize) -> Result<String, std::io::Error> {
    let path = PathBuf::from(format!("inputs/{y}/{:02?}.txt", d));
    std::fs::read_to_string(path)
}

#[macro_export]
macro_rules! solve {
    ($year:expr, $day:expr, $p1:ident, $p2:ident) => {
        pub fn solve() -> Result<(), aoc::Error> {
            let input = aoc::pull_input($year, $day)?;

            println!("{}", $p1(&input));
            println!("{}", $p2(&input));

            Ok(())
        }
    };
}
