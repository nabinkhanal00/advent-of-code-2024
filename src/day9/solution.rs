use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
pub fn read() -> Result<String, Box<dyn Error>> {
    let file = File::open("src/day9/input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut disk_map = String::new();
    reader.read_line(&mut disk_map)?;
    Ok(disk_map)
}
pub mod part1 {
    use std::{collections::HashMap, collections::HashSet, error::Error, io::BufRead};

    use super::read;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let disk_map = read()?;
        let mut disk_map: Vec<char> = disk_map.chars().collect();
        disk_map.remove(disk_map.len() - 1);
        let length = disk_map.len();
        let mut checksum = 0;
        let mut start = 0 as usize;
        let mut end = if length % 2 == 1 {
            length - 1
        } else {
            length - 2
        };
        let mut cur_start: i64 = 0;
        while start <= end {
            let s = disk_map[start];
            let s = s.to_digit(10).unwrap() as i64;
            if start % 2 == 0 {
                let id = (start / 2) as i64;
                checksum += id * s * (2 * cur_start + (s - 1)) / 2;
                cur_start += s;
                start += 1;
            } else {
                let id = (end / 2) as i64;
                let e = disk_map[end] as char;
                let e = e.to_digit(10).unwrap() as i64;
                if s == e {
                    checksum += id * s * (2 * cur_start + (s - 1)) / 2;
                    cur_start += s;
                    end = end - 2;
                    start += 1;
                } else if s < e {
                    checksum += id * s * (2 * cur_start + (s - 1)) / 2;
                    cur_start += s;
                    start += 1;
                    disk_map[end] = ((e - s) as u8 + b'0') as char;
                } else {
                    checksum += id * e * (2 * cur_start + (e - 1)) / 2;
                    cur_start += e;
                    disk_map[start] = ((s - e) as u8 + b'0') as char;
                    end -= 2;
                }
            }
        }
        print!("The checksum is {checksum}");
        Ok(())
    }
}

pub mod part2 {
    use std::{collections::HashMap, collections::HashSet, error::Error, io::BufRead};

    use super::read;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let disk_map = read()?;
        let mut disk_map: Vec<char> = disk_map.chars().collect();
        disk_map.remove(disk_map.len() - 1);
        let length = disk_map.len();
        let mut checksum = 0;
        let mut start = 0 as usize;
        let mut end = if length % 2 == 1 {
            length - 1
        } else {
            length - 2
        };
        let initial_end = end;
        let mut cur_start: i64 = 0;
        let mut moved = vec![false; length];
        while start < length {
            if moved[start] {
                let cs = disk_map[start];
                let s = cs.to_digit(10).unwrap() as i64;
                cur_start += s;
                start += 1;
                continue;
            }
            while start <= end {
                let cs = disk_map[start];
                let s = cs.to_digit(10).unwrap() as i64;
                if start % 2 == 0 {
                    let id = (start / 2) as i64;
                    checksum += id * s * (2 * cur_start + (s - 1)) / 2;
                    disk_map[start] = b'0' as char;
                    cur_start += s;
                    break;
                } else {
                    let id = (end / 2) as i64;
                    let e = disk_map[end] as char;
                    let e = e.to_digit(10).unwrap() as i64;
                    let skip = moved[end] || s < e;
                    if skip {
                        end -= 2;
                    } else if s == e {
                        checksum += id * s * (2 * cur_start + (s - 1)) / 2;
                        cur_start += s;
                        disk_map[start] = b'0' as char;
                        moved[end] = true;
                        break;
                    } else {
                        checksum += id * e * (2 * cur_start + (e - 1)) / 2;
                        cur_start += e;
                        disk_map[start] = ((s - e) as u8 + b'0') as char;
                        // println!("Adding {}", s - e);
                        moved[end] = true;
                        end -= 2;
                    }
                }
            }
            cur_start += disk_map[start].to_digit(10).unwrap() as i64;
            start += 1;
            end = initial_end;
        }
        println!("\nThe checksum is {checksum}");
        Ok(())
    }
}
