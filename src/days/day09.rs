use std::collections::HashMap;
use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day09.txt").unwrap();
    let heightmap: Vec<Vec<u32>> = stream
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let part1_sol = part1(&heightmap);
    let part2_sol = part2(&heightmap);

    (part1_sol, part2_sol)
}

fn part1(heightmap: &Vec<Vec<u32>>) -> String {
    return find_low_points(heightmap).0.to_string();
}

fn part2(heightmap: &Vec<Vec<u32>>) -> String {
    let low_points: Vec<(usize, usize)> = find_low_points(heightmap).1;

    let mut prod = Vec::new();

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            visited.insert((y, x), false);
        }
    }

    for low_point in low_points {
        prod.push(find_basin_size(
            heightmap,
            (low_point.0, low_point.1),
            &mut visited,
        ));
    }

    prod.sort_by(|a, b| b.cmp(a));

    return (prod[0] * prod[1] * prod[2]).to_string();
}

fn find_basin_size(
    heightmap: &Vec<Vec<u32>>,
    coord: (usize, usize),
    visited: &mut HashMap<(usize, usize), bool>,
) -> u32 {
    if heightmap[coord.0][coord.1] == 9 || visited[&coord] {
        return 0;
    }

    let mut sum = 1;
    *visited.get_mut(&coord).unwrap() = true;

    if coord.0 < heightmap.len() - 1 {
        sum += find_basin_size(heightmap, (coord.0 + 1, coord.1), visited);
    }
    if coord.0 > 0 {
        sum += find_basin_size(heightmap, (coord.0 - 1, coord.1), visited);
    }
    if coord.1 < heightmap[0].len() - 1 {
        sum += find_basin_size(heightmap, (coord.0, coord.1 + 1), visited);
    }
    if coord.1 > 0 {
        sum += find_basin_size(heightmap, (coord.0, coord.1 - 1), visited);
    }

    sum
}

fn find_low_points(heightmap: &Vec<Vec<u32>>) -> (u32, Vec<(usize, usize)>) {
    let mut sum = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let current = heightmap[y][x];
            if y != heightmap.len() - 1 {
                if current >= heightmap[y + 1][x] {
                    continue;
                }
            }
            if y != 0 {
                if current >= heightmap[y - 1][x] {
                    continue;
                }
            }
            if x != 0 {
                if current >= heightmap[y][x - 1] {
                    continue;
                }
            }
            if x != heightmap[y].len() - 1 {
                if current >= heightmap[y][x + 1] {
                    continue;
                }
            }
            sum += current + 1;
            low_points.push((y, x));
        }
    }
    (sum, low_points)
}
