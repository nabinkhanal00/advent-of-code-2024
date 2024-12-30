use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("src/day04/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        let line = line?;
        content.push(line);
    }
    Ok(content)
}
pub mod part1 {
    use std::error::Error;

    use super::read;

    fn check_down(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x].chars().nth(y + 1).unwrap() == 'M'
            && content[x].chars().nth(y + 2).unwrap() == 'A'
            && content[x].chars().nth(y + 3).unwrap() == 'S'
    }
    fn check_up(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x].chars().nth(y - 1).unwrap() == 'M'
            && content[x].chars().nth(y - 2).unwrap() == 'A'
            && content[x].chars().nth(y - 3).unwrap() == 'S'
    }
    fn check_right(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x + 1].chars().nth(y).unwrap() == 'M'
            && content[x + 2].chars().nth(y).unwrap() == 'A'
            && content[x + 3].chars().nth(y).unwrap() == 'S'
    }
    fn check_left(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x - 1].chars().nth(y).unwrap() == 'M'
            && content[x - 2].chars().nth(y).unwrap() == 'A'
            && content[x - 3].chars().nth(y).unwrap() == 'S'
    }
    fn check_ur(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x + 1].chars().nth(y - 1).unwrap() == 'M'
            && content[x + 2].chars().nth(y - 2).unwrap() == 'A'
            && content[x + 3].chars().nth(y - 3).unwrap() == 'S'
    }
    fn check_ul(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x - 1].chars().nth(y - 1).unwrap() == 'M'
            && content[x - 2].chars().nth(y - 2).unwrap() == 'A'
            && content[x - 3].chars().nth(y - 3).unwrap() == 'S'
    }
    fn check_dr(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x + 1].chars().nth(y + 1).unwrap() == 'M'
            && content[x + 2].chars().nth(y + 2).unwrap() == 'A'
            && content[x + 3].chars().nth(y + 3).unwrap() == 'S'
    }
    fn check_dl(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'X'
            && content[x - 1].chars().nth(y + 1).unwrap() == 'M'
            && content[x - 2].chars().nth(y + 2).unwrap() == 'A'
            && content[x - 3].chars().nth(y + 3).unwrap() == 'S'
    }

    fn is_xmas(content: &Vec<String>, x: i64, y: i64) -> usize {
        let length = content.len() as i64;
        let mut count = 0;
        if y + 3 < length && check_down(content, x as usize, y as usize) {
            count += 1;
        }
        if y - 3 >= 0 && check_up(content, x as usize, y as usize) {
            count += 1;
        }
        if x + 3 < length && check_right(content, x as usize, y as usize) {
            count += 1;
        }
        if x - 3 >= 0 && check_left(content, x as usize, y as usize) {
            count += 1;
        }
        if x + 3 < length && y - 3 >= 0 && check_ur(content, x as usize, y as usize) {
            count += 1;
        }
        if x + 3 < length && y + 3 < length && check_dr(content, x as usize, y as usize) {
            count += 1;
        }
        if x - 3 >= 0 && y - 3 >= 0 && check_ul(content, x as usize, y as usize) {
            count += 1;
        }
        if x - 3 >= 0 && y + 3 < length && check_dl(content, x as usize, y as usize) {
            count += 1;
        }
        count
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let content = read()?;
        let mut count = 0;
        let length = content.len();
        for i in 0..length {
            for j in 0..length {
                count += is_xmas(&content, i as i64, j as i64);
            }
        }
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::error::Error;
    fn is_mas(content: &Vec<String>, x: usize, y: usize) -> bool {
        content[x].chars().nth(y).unwrap() == 'A'
            && ((content[x - 1].chars().nth(y - 1).unwrap() == 'M'
                && content[x + 1].chars().nth(y + 1).unwrap() == 'S')
                || (content[x - 1].chars().nth(y - 1).unwrap() == 'S'
                    && content[x + 1].chars().nth(y + 1).unwrap() == 'M'))
            && ((content[x - 1].chars().nth(y + 1).unwrap() == 'M'
                && content[x + 1].chars().nth(y - 1).unwrap() == 'S')
                || (content[x - 1].chars().nth(y + 1).unwrap() == 'S'
                    && content[x + 1].chars().nth(y - 1).unwrap() == 'M'))
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let content = read()?;
        let mut count = 0;
        let length = content.len();
        for i in 1..length - 1 {
            for j in 1..length - 1 {
                if is_mas(&content, i, j) {
                    println!("{i} {j}");
                    count += 1
                }
            }
        }
        println!("Total count is: {count}");
        Ok(())
    }
}
