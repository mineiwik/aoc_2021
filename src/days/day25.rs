use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day25.txt").unwrap();
    let part1_sol = part1();
    let part2_sol = part2();

    (part1_sol, part2_sol)
}

fn part1() -> String {
    "part1".to_string()
}

fn part2() -> String {
    "part2".to_string()
}
