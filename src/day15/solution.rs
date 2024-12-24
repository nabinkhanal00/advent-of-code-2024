use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<(Vec<Vec<char>>, (usize, usize), String), Box<dyn Error>> {
    let file = File::open("src/day15/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut is_grid = true;
    let mut grid = Vec::new();
    let mut directions = String::new();
    let mut pos = None;
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        if line == "" {
            is_grid = false;
            continue;
        }
        if is_grid {
            if pos.is_none() {
                if let Some(x) = line.find("@") {
                    pos = Some((y, x));
                }
            }
            grid.push(line.chars().collect());
        } else {
            directions.push_str(line.as_str());
        }
    }
    Ok((grid, pos.unwrap(), directions))
}

pub mod part1 {
    use super::read;
    use std::error::Error;
    fn print_grid(grid: &Vec<Vec<char>>) {
        for row in grid {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
    fn find_sum(grid: &Vec<Vec<char>>) -> i64 {
        let mut sum = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'O' {
                    sum += y * 100 + x;
                }
            }
        }
        sum as i64
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let (mut grid, mut pos, directions) = read()?;
        for dir in directions.chars() {
            match dir {
                '>' => {
                    let (y, mut x) = pos;

                    while grid[y][x] == 'O' || grid[y][x] == '@' {
                        x += 1;
                    }
                    if grid[y][x] != '#' {
                        while x > pos.1 {
                            grid[y][x] = grid[y][x - 1];
                            x -= 1;
                        }
                        grid[pos.0][pos.1] = '.';
                        pos = (y, x + 1);
                    }
                }
                '<' => {
                    let (y, mut x) = pos;
                    while grid[y][x] == 'O' || grid[y][x] == '@' {
                        x -= 1;
                    }
                    if grid[y][x] != '#' {
                        while x < pos.1 {
                            grid[y][x] = grid[y][x + 1];
                            x += 1;
                        }
                        grid[pos.0][pos.1] = '.';
                        pos = (y, x - 1);
                    }
                }
                '^' => {
                    let (mut y, x) = pos;
                    while grid[y][x] == 'O' || grid[y][x] == '@' {
                        y -= 1;
                    }
                    if grid[y][x] != '#' {
                        while y < pos.0 {
                            grid[y][x] = grid[y + 1][x];
                            y += 1;
                        }
                        grid[pos.0][pos.1] = '.';
                        pos = (y - 1, x);
                    }
                }
                'v' => {
                    let (mut y, x) = pos;
                    while grid[y][x] == 'O' || grid[y][x] == '@' {
                        y += 1;
                    }
                    if grid[y][x] != '#' {
                        while y > pos.0 {
                            grid[y][x] = grid[y - 1][x];
                            y -= 1;
                        }
                        grid[pos.0][pos.1] = '.';
                        pos = (y + 1, x);
                    }
                }
                _ => {
                    unimplemented!()
                }
            }
            // println!("Move: {dir}");
            // print_grid(&grid);
            // println!("",);
        }
        println!("The sum is: {}", find_sum(&grid));
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::error::Error;
    #[derive(PartialEq, Clone, Copy)]
    enum Element {
        Barrier,
        StartBox,
        EndBox,
        Me,
        Empty,
    }
    fn adjust(grid: Vec<Vec<char>>) -> (Vec<Vec<Element>>, (usize, usize)) {
        let mut res = Vec::new();
        let mut pos = (-1, -1);
        for (y, row) in grid.iter().enumerate() {
            let mut r = Vec::new();
            for (x, c) in row.iter().enumerate() {
                match c {
                    '#' => {
                        r.push(Element::Barrier);
                        r.push(Element::Barrier);
                    }
                    '.' => {
                        r.push(Element::Empty);
                        r.push(Element::Empty);
                    }
                    'O' => {
                        r.push(Element::StartBox);
                        r.push(Element::EndBox);
                    }
                    '@' => {
                        pos = (y as i64, (2 * x) as i64);
                        r.push(Element::Me);
                        r.push(Element::Empty);
                    }
                    _ => unimplemented!(),
                }
            }
            res.push(r);
        }
        (res, (pos.0 as usize, pos.1 as usize))
    }

    fn move_up(pos: (usize, usize), grid: &mut Vec<Vec<Element>>) {
        let (y, x) = pos;
        match grid[y][x] {
            Element::Barrier => {}
            Element::Empty => {}
            Element::Me => {
                move_up((y - 1, x), grid);
                grid[y - 1][x] = Element::Me;
                grid[y][x] = Element::Empty;
            }
            Element::StartBox => {
                move_up((y - 1, x), grid);
                move_up((y - 1, x + 1), grid);
                grid[y - 1][x] = Element::StartBox;
                grid[y - 1][x + 1] = Element::EndBox;
                grid[y][x] = Element::Empty;
                grid[y][x + 1] = Element::Empty;
            }
            Element::EndBox => {
                move_up((y - 1, x), grid);
                move_up((y - 1, x - 1), grid);
                grid[y - 1][x] = Element::EndBox;
                grid[y - 1][x - 1] = Element::StartBox;
                grid[y][x] = Element::Empty;
                grid[y][x - 1] = Element::Empty;
            }
        };
    }
    fn can_move_up(pos: (usize, usize), grid: &Vec<Vec<Element>>) -> bool {
        let (y, x) = pos;
        match grid[y][x] {
            Element::Barrier => false,
            Element::Empty => true,
            Element::Me => can_move_up((y - 1, x), grid),
            Element::StartBox => can_move_up((y - 1, x), grid) && can_move_up((y - 1, x + 1), grid),
            Element::EndBox => can_move_up((y - 1, x), grid) && can_move_up((y - 1, x - 1), grid),
        }
    }

    fn move_down(pos: (usize, usize), grid: &mut Vec<Vec<Element>>) {
        let (y, x) = pos;
        match grid[y][x] {
            Element::Barrier => {}
            Element::Empty => {}
            Element::Me => {
                move_down((y + 1, x), grid);
                grid[y + 1][x] = Element::Me;
                grid[y][x] = Element::Empty;
            }
            Element::StartBox => {
                move_down((y + 1, x), grid);
                move_down((y + 1, x + 1), grid);
                grid[y + 1][x] = Element::StartBox;
                grid[y + 1][x + 1] = Element::EndBox;
                grid[y][x] = Element::Empty;
                grid[y][x + 1] = Element::Empty;
            }
            Element::EndBox => {
                move_down((y + 1, x), grid);
                move_down((y + 1, x - 1), grid);
                grid[y + 1][x] = Element::EndBox;
                grid[y + 1][x - 1] = Element::StartBox;
                grid[y][x] = Element::Empty;
                grid[y][x - 1] = Element::Empty;
            }
        };
    }
    fn can_move_down(pos: (usize, usize), grid: &Vec<Vec<Element>>) -> bool {
        let (y, x) = pos;
        match grid[y][x] {
            Element::Barrier => false,
            Element::Empty => true,
            Element::Me => can_move_down((y + 1, x), grid),
            Element::StartBox => {
                can_move_down((y + 1, x), grid) && can_move_down((y + 1, x + 1), grid)
            }
            Element::EndBox => {
                can_move_down((y + 1, x), grid) && can_move_down((y + 1, x - 1), grid)
            }
        }
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let (grid, _, directions) = read()?;
        let (mut grid, mut pos) = adjust(grid);

        for dir in directions.chars() {
            match dir {
                '<' => {
                    let (y, mut x) = pos;
                    x -= 1;
                    while grid[y][x] != Element::Empty && grid[y][x] != Element::Barrier {
                        x -= 1;
                    }
                    if grid[y][x] != Element::Barrier {
                        while x < pos.1 {
                            grid[y][x] = grid[y][x + 1];
                            x += 1;
                        }
                        grid[pos.0][pos.1] = Element::Empty;
                        pos = (y, x - 1);
                    }
                }
                '>' => {
                    let (y, mut x) = pos;
                    x += 1;
                    while grid[y][x] != Element::Empty && grid[y][x] != Element::Barrier {
                        x += 1;
                    }
                    if grid[y][x] != Element::Barrier {
                        while x > pos.1 {
                            grid[y][x] = grid[y][x - 1];
                            x -= 1;
                        }
                        grid[pos.0][pos.1] = Element::Empty;
                        pos = (y, x + 1);
                    }
                }
                '^' => {
                    if can_move_up(pos, &grid) {
                        move_up(pos, &mut grid);
                        pos = (pos.0 - 1, pos.1);
                    }
                }
                'v' => {
                    if can_move_down(pos, &grid) {
                        move_down(pos, &mut grid);
                        pos = (pos.0 + 1, pos.1);
                    }
                }
                _ => unimplemented!(),
            }
        }

        let mut sum = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == Element::StartBox {
                    sum += y * 100 + x;
                }
            }
        }
        println!("The sum is: {sum}");
        Ok(())
    }
}
