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
    // Calculating average (int(avg) +- 1)
    let mut avg_sum = 0;
    let mut lower_sum = 0;
    let mut upper_sum = 0;
    let avg = crabs.iter().sum::<isize>() as isize / crabs.len() as isize;
    let upper_avg = avg + 1;
    let lower_avg = avg - 1;
    for crab in crabs {
        let avg_dif = (avg - crab).abs();
        let upper_dif = (upper_avg - crab).abs();
        let lower_dif = (lower_avg - crab).abs();
        avg_sum += avg_dif * (avg_dif + 1) / 2;
        upper_sum += upper_dif * (upper_dif + 1) / 2;
        lower_sum += lower_dif * (lower_dif + 1) / 2;
    }

    return std::cmp::min(avg_sum, std::cmp::min(lower_sum, upper_sum)).to_string();
}
