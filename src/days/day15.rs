use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Level {
    risk: usize,
    position: (usize, usize),
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day15.txt").unwrap();
    let lines = stream.trim().lines();

    let mut risk_levels: HashMap<(usize, usize), usize> = HashMap::new();

    let width = lines.clone().collect::<Vec<&str>>().len();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            risk_levels.insert((y, x), c.to_digit(10).unwrap() as usize);
        }
    }
    let part1_sol = part1(&risk_levels, width);
    let part2_sol = part2(&risk_levels, width);

    (part1_sol, part2_sol)
}

fn part1(risk_levels: &HashMap<(usize, usize), usize>, width: usize) -> String {
    let shortest = shortest_path(&risk_levels, width, (0, 0), (width - 1, width - 1));
    return shortest.to_string();
}

fn part2(risk_levels: &HashMap<(usize, usize), usize>, width: usize) -> String {
    let mut risk_levels = risk_levels.clone();
    for i in 1..5 {
        for y in 0..width {
            for x in 0..width {
                let mut next_risk_level = *risk_levels.get(&(y, x)).unwrap() + i;
                next_risk_level = if next_risk_level > 9 {
                    next_risk_level - 9
                } else {
                    next_risk_level
                };
                risk_levels.insert((y, x + i * width), next_risk_level);
            }
        }
    }

    for i in 1..5 {
        for x in 0..5 * width {
            for y in 0..width {
                let mut next_risk_level = *risk_levels.get(&(y, x)).unwrap() + i;
                next_risk_level = if next_risk_level > 9 {
                    next_risk_level - 9
                } else {
                    next_risk_level
                };
                risk_levels.insert((y + i * width, x), next_risk_level);
            }
        }
    }

    let shortest = shortest_path(
        &risk_levels,
        5 * width,
        (0, 0),
        (5 * width - 1, 5 * width - 1),
    );
    return shortest.to_string();
}

fn shortest_path(
    risk_levels: &HashMap<(usize, usize), usize>,
    width: usize,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();

    for i in 0..width {
        for j in 0..width {
            dist.insert((i, j), usize::MAX);
        }
    }

    let mut heap = BinaryHeap::new();

    *dist.entry(start).or_insert(usize::MAX) = 0;

    heap.push(Level {
        risk: 0,
        position: start,
    });

    while let Some(Level { risk, position }) = heap.pop() {
        if position == goal {
            return risk;
        }

        // Important as we may have already found a better way
        if risk > *dist.get(&position).unwrap() {
            continue;
        }

        for y in std::cmp::max(position.0 as isize - 1, 0)
            ..std::cmp::min(position.0 as isize + 2, width as isize)
        {
            let next = Level {
                risk: risk + risk_levels.get(&(y as usize, position.1 as usize)).unwrap(),
                position: (y as usize, position.1),
            };
            if next.risk < *dist.get(&next.position).unwrap() {
                heap.push(next);
                *dist.entry(next.position).or_insert(usize::MAX) = next.risk;
            }
        }
        for x in std::cmp::max(position.1 as isize - 1, 0)
            ..std::cmp::min(position.1 as isize + 2, width as isize)
        {
            let next = Level {
                risk: risk + risk_levels.get(&(position.0 as usize, x as usize)).unwrap(),
                position: (position.0, x as usize),
            };
            if next.risk < *dist.get(&next.position).unwrap() {
                heap.push(next);
                *dist.entry(next.position).or_insert(usize::MAX) = next.risk;
            }
        }
    }
    0
}
