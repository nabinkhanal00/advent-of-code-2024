use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<(Vec<String>, (i64, i64)), Box<dyn Error>> {
    let file = File::open("src/day06/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut content = Vec::new();
    let mut x: i64 = -1;
    let mut y: i64 = -1;
    for (_y, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some(pos) = line.find("^") {
            y = _y as i64;
            x = pos as i64;
        }

        content.push(line);
    }
    Ok((content, (y, x)))
}
pub mod part1 {
    use std::{collections, error::Error};

    use super::read;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let (content, (mut y, mut x)) = read()?;
        let mut visited = collections::HashSet::new();
        let mut xdiff: i64 = 0;
        let mut ydiff: i64 = -1;

        let mut count: i64 = 0;
        while x >= 0 && y >= 0 && x < content.len() as i64 && y < content.len() as i64 {
            if (x + xdiff) >= 0
                && (y + ydiff) >= 0
                && (x + xdiff) < content.len() as i64
                && (y + ydiff) < content.len() as i64
                && content[(y + ydiff) as usize]
                    .chars()
                    .nth((x + xdiff) as usize)
                    .unwrap()
                    == '#'
            {
                (xdiff, ydiff) = (ydiff * -1, xdiff);
            } else {
                let inserted = visited.insert((y, x));
                if inserted {
                    count += 1;
                }
                x += xdiff;
                y += ydiff;
            }
        }

        println!("The count of visited places is: {count}");
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::collections;
    use std::error::Error;

    pub fn is_loop(content: &Vec<String>, o: (i64, i64), pos: (i64, i64)) -> bool {
        let mut visited = collections::HashSet::new();
        let mut xdiff: i64 = 0;
        let mut ydiff: i64 = -1;

        let mut y = pos.0;
        let mut x = pos.1;
        while x >= 0 && y >= 0 && x < content.len() as i64 && y < content.len() as i64 {
            if ((x + xdiff) >= 0
                && (y + ydiff) >= 0
                && (x + xdiff) < content.len() as i64
                && (y + ydiff) < content.len() as i64
                && content[(y + ydiff) as usize]
                    .chars()
                    .nth((x + xdiff) as usize)
                    .unwrap()
                    == '#')
                || (y + ydiff, x + xdiff) == o
            {
                (xdiff, ydiff) = (ydiff * -1, xdiff);
            } else {
                let new = visited.insert(((y, x), (ydiff, xdiff)));
                if !new {
                    return true;
                }
                x += xdiff;
                y += ydiff;
            }
        }
        false
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let (content, (y, x)) = read()?;
        let mut count = 0;

        for yc in 0..content.len() {
            for xc in 0..content.len() {
                if content[yc].chars().nth(xc).unwrap() == '#' || (yc as i64, xc as i64) == (y, x) {
                    continue;
                }
                let loops = is_loop(&content, (yc as i64, xc as i64), (y, x));
                if loops {
                    count += 1;
                }
            }
        }
        println!("The count of possible loops are: {count}");
        Ok(())
    }
}
