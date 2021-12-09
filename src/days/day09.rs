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

    let mut res: (u32, Vec<(usize, usize)>);
    let mut prod = Vec::new();
    for low_point in low_points {
        res = find_basin_size(heightmap, (low_point.0, low_point.1), Vec::new());
        prod.push(res.0);
    }

    prod.sort_by(|a, b| b.cmp(a));

    return (prod[0] * prod[1] * prod[2]).to_string();
}

fn find_basin_size(
    heightmap: &Vec<Vec<u32>>,
    coord: (usize, usize),
    mut visited: Vec<(usize, usize)>,
) -> (u32, Vec<(usize, usize)>) {
    if heightmap[coord.0][coord.1] == 9 || visited.contains(&coord) {
        return (0, visited);
    }

    let mut sum = 1;
    let mut res: (u32, Vec<(usize, usize)>);
    visited.push(coord);

    if coord.0 < heightmap.len() - 1 {
        res = find_basin_size(heightmap, (coord.0 + 1, coord.1), visited);
        sum += res.0;
        visited = res.1;
    }
    if coord.0 > 0 {
        res = find_basin_size(heightmap, (coord.0 - 1, coord.1), visited);
        sum += res.0;
        visited = res.1;
    }
    if coord.1 < heightmap[0].len() - 1 {
        res = find_basin_size(heightmap, (coord.0, coord.1 + 1), visited);
        sum += res.0;
        visited = res.1;
    }
    if coord.1 > 0 {
        res = find_basin_size(heightmap, (coord.0, coord.1 - 1), visited);
        sum += res.0;
        visited = res.1;
    }

    (sum, visited)
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
