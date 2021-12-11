use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day11.txt").unwrap();
    let octopuses: Vec<Vec<u32>> = stream
        .trim()
        .lines()
        .map(|row| {
            row.chars()
                .map(|column| column.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let part1_sol = part1(&octopuses);
    let part2_sol = part2(&octopuses);

    (part1_sol, part2_sol)
}

fn part1(octopuses: &Vec<Vec<u32>>) -> String {
    return sim_flashes(octopuses).0.to_string();
}

fn part2(octopuses: &Vec<Vec<u32>>) -> String {
    return sim_flashes(octopuses).1.to_string();
}

fn sim_flashes(octopuses: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut total_flashes: u32 = 0;
    let mut steps = 0;
    let mut sim_octopuses = octopuses.clone();

    loop {
        steps += 1;
        for i in 0..sim_octopuses.len() {
            for j in 0..sim_octopuses[0].len() {
                sim_octopuses[i][j] += 1;
            }
        }
        let mut step_flashes = 0;
        loop {
            let mut new_flashes = 0;
            for i in 0..sim_octopuses.len() {
                for j in 0..sim_octopuses[0].len() {
                    if sim_octopuses[i][j] > 9 {
                        new_flashes += 1;
                        sim_octopuses[i][j] = 0;
                        for k in (i as isize - 1)..(i as isize + 2) {
                            for l in (j as isize - 1)..(j as isize + 2) {
                                if k == i as isize && l == j as isize
                                    || k < 0
                                    || l < 0
                                    || k > sim_octopuses.len() as isize - 1
                                    || l > sim_octopuses[0].len() as isize - 1
                                {
                                    continue;
                                }
                                if sim_octopuses[k as usize][l as usize] != 0 {
                                    sim_octopuses[k as usize][l as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            if new_flashes == 0 {
                break;
            }
            step_flashes += new_flashes;
        }
        if steps <= 100 {
            total_flashes += step_flashes;
        }
        if step_flashes == sim_octopuses.len() as u32 * sim_octopuses[0].len() as u32 {
            break;
        }
    }

    (total_flashes, steps)
}
