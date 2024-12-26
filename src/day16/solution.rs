use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<(HashSet<(i64, i64)>, (i64, i64), (i64, i64)), Box<dyn Error>> {
    let file = File::open("src/day16/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut vertices = HashSet::new();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            let pos = (y as i64, x as i64);
            if c == '.' {
                vertices.insert(pos);
            } else if c == 'S' {
                start = pos;
            } else if c == 'E' {
                vertices.insert(pos);
                end = pos;
            }
        }
    }
    Ok((vertices, start, end))
}

pub mod part1 {
    use super::read;
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        error::Error,
        usize,
    };
    const DIFFS: [(i64, i64); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    pub fn path() -> Result<i64, Box<dyn Error>> {
        let (vertices, start, end) = read()?;
        let mut distance = HashMap::new();
        let mut previous = HashMap::new();
        let mut q = HashSet::new();
        for vertex in vertices {
            distance.insert(vertex, usize::MAX);
            q.insert(vertex);
        }
        q.insert(start);
        distance.insert(start, 0);
        previous.insert(start, (0, 1));
        while q.len() > 0 {
            let el = q.iter().min_by_key(|e| distance.get(*e)).unwrap().clone();
            q.remove(&el);
            let el_distance = *distance.get(&el).unwrap();
            for diff in DIFFS {
                let neighbour = (el.0 + diff.0, el.1 + diff.1);
                if q.contains(&neighbour) {
                    let dist;
                    if diff == previous.get(&el).unwrap().clone() {
                        dist = 1;
                    } else {
                        dist = 1001;
                    }
                    let dist = dist + el_distance;
                    if dist < *distance.get(&neighbour).unwrap() {
                        distance.insert(neighbour, dist);
                        previous.insert(neighbour, diff);
                    }
                }
            }
        }
        Ok(distance.get(&end).unwrap().clone() as i64)
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        println!("The shortest path is: {}", path()?);
        Ok(())
    }
}

pub mod part2 {
    use super::part1::path;
    use super::read;
    use std::{
        cmp::min_by,
        collections::{BinaryHeap, HashMap, HashSet, VecDeque},
        error::Error,
    };
    const DIFFS: [(i64, i64); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    #[derive(Debug, Eq, PartialEq)]
    struct Element {
        pos: (i64, i64),
        dir: (i64, i64),
        cost: i64,
    }
    impl Ord for Element {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    impl PartialOrd for Element {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let (vertices, start, end) = read()?;
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        queue.push(Element {
            pos: start,
            dir: (0, 1),
            cost: 0,
        });
        let mut previous = HashMap::new();
        while let Some(el) = queue.pop() {
            if visited.contains(&(el.pos, el.dir)) {
                continue;
            }
            visited.insert((el.pos, el.dir));
            for diff in DIFFS {
                let e = el.pos;
                let dir = el.dir;
                if dir.0 == -diff.0 && dir.1 == -diff.1 {
                    continue;
                }
                let n = (e.0 + diff.0, e.1 + diff.1);
                let cost: i64;
                if dir == diff {
                    cost = 1;
                } else {
                    cost = 1001;
                }
                if vertices.contains(&n) && !visited.contains(&(n, diff)) {
                    let prev_el = previous.entry(n).or_insert(Vec::new());
                    prev_el.push(((el.pos, dir), el.cost));
                    queue.push(Element {
                        pos: n,
                        dir: diff,
                        cost: cost + el.cost,
                    });
                }
            }
        }
        let mut final_queue = VecDeque::new();
        let shortest_dist = previous
            .get(&end)
            .unwrap()
            .iter()
            .map(|v| {
                let cost = v.1;
                let point = v.0 .0;
                let dir = v.0 .1;
                let new_dir = (end.0 - point.0, end.1 - point.1);
                let additional_cost;
                if new_dir == dir {
                    additional_cost = 1;
                } else {
                    additional_cost = 1001;
                }
                cost + additional_cost
            })
            .min()
            .unwrap();
        final_queue.push_back((end, previous.get(&end).unwrap(), shortest_dist));
        let mut vertices = HashSet::new();
        vertices.insert(end);
        while let Some((to_point, path, dist)) = final_queue.pop_front() {
            for ((from_point, dir), cost) in path {
                let new_dir = (to_point.0 - from_point.0, to_point.1 - from_point.1);
                let additional_cost;
                if new_dir == *dir {
                    additional_cost = 1;
                } else {
                    additional_cost = 1001;
                }
                if cost + additional_cost == dist {
                    vertices.insert(*from_point);
                    if *from_point == start {
                        continue;
                    }
                    final_queue.push_back((*from_point, previous.get(from_point).unwrap(), *cost));
                }
            }
        }

        println!(
            "Shortest dist: {}, Unique vertices: {}",
            shortest_dist,
            vertices.len()
        );

        Ok(())
    }
}
