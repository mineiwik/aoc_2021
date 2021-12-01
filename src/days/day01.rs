use std::fs;

pub fn solve() -> (String, String) {
    let stream: String = fs::read_to_string("inputs/day01.txt").unwrap();
    let depths: Vec<usize> = stream
        .split("\n")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    let part1_sol = part1(&depths);
    let part2_sol = part2(&depths);

    (part1_sol, part2_sol)
}

fn part1(depths: &Vec<usize>) -> String {
    return count_ascendings(depths).to_string();
}

fn part2(depths: &Vec<usize>) -> String {
    let depths_window: Vec<usize> = depths.windows(3).map(|i| i.iter().sum()).collect();
    return count_ascendings(&depths_window).to_string();
}

fn count_ascendings(depths: &Vec<usize>) -> usize {
    depths.windows(2).filter(|i| i[1] > i[0]).count()
}
