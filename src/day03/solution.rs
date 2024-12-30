use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};
pub fn read() -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open("src/day03/input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    Ok(content)
}
pub mod part1 {
    use std::error::Error;

    use super::read;
    use regex::Regex;
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let content = read()?;
        let re = Regex::new(r"mul\((\b\d{1,3}\b),(\b\d{1,3}\b)\)")?;

        let mut sum = 0;
        for (_, [first, second]) in re
            .captures_iter(String::from_utf8(content)?.as_str())
            .map(|c| c.extract())
        {
            let first: i64 = first.parse()?;
            let second: i64 = second.parse()?;
            sum += first * second;
        }

        println!("The sum is: {sum}");
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use regex::{Error, Regex};

    pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
        let content = read()?;
        let re = Regex::new(r"mul\((\b\d{1,3}\b),(\b\d{1,3}\b)\)|((?:do|don't)\(\))")?;

        let mut donot = false;
        let mut sum = 0;
        for cmatch in re.captures_iter(String::from_utf8(content)?.as_str()) {
            if let Some(mul) = cmatch.get(3) {
                let value = mul.as_str();
                if value == "don't()" {
                    donot = true;
                } else {
                    donot = false;
                }
            } else {
                let first: i64 = cmatch
                    .get(1)
                    .ok_or(Error::Syntax("first group not matching".to_string()))?
                    .as_str()
                    .parse()?;
                let second: i64 = cmatch
                    .get(2)
                    .ok_or(Error::Syntax("couldn't capture first number".to_string()))?
                    .as_str()
                    .parse()?;
                if !donot {
                    sum += first * second;
                }
            }
        }
        println!("The sum is: {sum}");

        Ok(())
    }
}
