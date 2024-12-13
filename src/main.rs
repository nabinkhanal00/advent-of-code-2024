use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), Box<dyn Error>> {
    // day1::part1::solve()?;
    // day1::part1::solve()?;
    // day2::part1::solve()?;
    // day2::part2::solve()?;
    // day3::part1::solve()?;
    // day3::part2::solve()?;
    // day4::part1::solve()?;
    // day4::part2::solve()?;
    // day5::part1::solve()?;
    // day6::part1::solve()?;
    day6::part2::solve()?;
    Ok(())
}
