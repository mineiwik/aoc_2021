use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day07.txt").unwrap();
    let crabs: Vec<isize> = stream
        .trim()
        .split(",")
        .map(|crab| crab.parse::<isize>().unwrap())
        .collect();

    let part1_sol = part1(&crabs);
    let part2_sol = part2(&crabs);

    (part1_sol, part2_sol)
}

fn part1(crabs: &Vec<isize>) -> String {
    // Calculating median
    let mut sum = 0;
    let mut crabs_sorted = crabs.clone();
    crabs_sorted.sort();
    let median = crabs_sorted[crabs_sorted.len() / 2];
    for crab in crabs {
        sum += (median - crab).abs();
    }
    return sum.to_string();
}

fn part2(crabs: &Vec<isize>) -> String {
    // Calculating average (floor and ceil values)
    let mut ceil_sum = 0;
    let mut floor_sum = 0;
    let floor_avg = crabs.iter().sum::<isize>() as isize / crabs.len() as isize;
    let ceil_avg = floor_avg + 1;
    for crab in crabs {
        let floor_dif = (floor_avg - crab).abs();
        let ceil_dif = (ceil_avg - crab).abs();
        ceil_sum += ceil_dif * (ceil_dif + 1) / 2;
        floor_sum += floor_dif * (floor_dif + 1) / 2;
    }

    let sum = if floor_sum < ceil_sum {
        floor_sum
    } else {
        ceil_sum
    };

    return sum.to_string();
}
