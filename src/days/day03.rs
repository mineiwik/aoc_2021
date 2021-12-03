use std::fs;

pub fn solve() -> (String, String) {
    let stream: String = fs::read_to_string("inputs/day03.txt").unwrap();
    let diagnostics: Vec<Vec<u32>> = stream
        .lines()
        .map(|i| i.chars().flat_map(|j| j.to_digit(10)).collect())
        .collect();

    let part1_sol = part1(&diagnostics);
    let part2_sol = part2(&diagnostics);

    (part1_sol, part2_sol)
}

fn part1(diagnostics: &Vec<Vec<u32>>) -> String {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..diagnostics[0].len() {
        let current_most_common = most_common(&get_column(diagnostics, i));
        let current_least_common = least_common(&get_column(diagnostics, i));
        gamma += 2u32.pow(diagnostics[0].len() as u32 - 1 - i as u32) * current_most_common;
        epsilon += 2u32.pow(diagnostics[0].len() as u32 - 1 - i as u32) * current_least_common;
    }

    return (gamma * epsilon).to_string();
}

fn part2(diagnostics: &Vec<Vec<u32>>) -> String {
    let mut rem = diagnostics.clone();

    let mut oxygen_rating = 0;
    let mut co2_rating = 0;

    for i in 0..diagnostics[0].len() {
        let current_most_common = most_common(&get_column(&rem, i));
        rem.retain(|j| j[i] == current_most_common);
        if rem.len() <= 1 {
            break;
        }
    }

    for (i, bit) in rem[0].iter().enumerate() {
        oxygen_rating += bit * 2u32.pow(diagnostics[0].len() as u32 - 1 - i as u32);
    }

    let mut rem = diagnostics.clone();

    for i in 0..diagnostics[0].len() {
        let current_most_common = least_common(&get_column(&rem, i));
        rem.retain(|j| j[i] == current_most_common);
        if rem.len() <= 1 {
            break;
        }
    }

    for (i, bit) in rem[0].iter().enumerate() {
        co2_rating += bit * 2u32.pow(diagnostics[0].len() as u32 - 1 - i as u32);
    }

    return (oxygen_rating * co2_rating).to_string();
}

fn get_column(rem: &Vec<Vec<u32>>, idx: usize) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::new();
    for row in rem {
        list.push(row[idx]);
    }

    list
}

fn most_common(list: &Vec<u32>) -> u32 {
    let mut sum: i32 = 0;
    for elem in list {
        sum += if *elem == 1u32 { 1 } else { -1 };
    }

    if sum >= 0 {
        1
    } else {
        0
    }
}

fn least_common(list: &Vec<u32>) -> u32 {
    if most_common(list) == 1 {
        0
    } else {
        1
    }
}
