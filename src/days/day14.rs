use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day14.txt").unwrap();
    let lines: Vec<&str> = stream.trim().split("\n\n").collect();

    let polymer_template: String = lines[0].trim().to_string();

    let pair_insertions: HashMap<(char, char), char> = lines[1]
        .trim()
        .lines()
        .map(|line| {
            let mut c_line = line.split(" -> ");
            let left: Vec<char> = c_line.next().unwrap().chars().collect();
            let right = c_line.next().unwrap().chars().next().unwrap();
            ((left[0], left[1]), right)
        })
        .into_iter()
        .collect();

    let part1_sol = part1(&polymer_template, &pair_insertions);
    let part2_sol = part2(&polymer_template, &pair_insertions);

    (part1_sol, part2_sol)
}

fn part1(polymer_template: &str, instructions: &HashMap<(char, char), char>) -> String {
    let mut counter: HashMap<char, u64> = HashMap::new();
    let mut dyn_counter: HashMap<(char, char, u64), HashMap<char, u64>> = HashMap::new();
    let mut last_b = '_';
    for (a, b) in polymer_template.chars().tuple_windows() {
        rec_count((a, b), instructions, &mut counter, 10, &mut dyn_counter);
        last_b = b;
    }

    *counter.entry(last_b).or_insert(0) += 1;

    let max = counter.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min = counter.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    return (max - min).to_string();
}

fn part2(polymer_template: &str, instructions: &HashMap<(char, char), char>) -> String {
    let mut counter: HashMap<char, u64> = HashMap::new();
    let mut dyn_counter: HashMap<(char, char, u64), HashMap<char, u64>> = HashMap::new();
    let mut last_b = '_';
    for (a, b) in polymer_template.chars().tuple_windows() {
        rec_count((a, b), instructions, &mut counter, 40, &mut dyn_counter);
        last_b = b;
    }

    *counter.entry(last_b).or_insert(0) += 1;

    let max = counter.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min = counter.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    return (max - min).to_string();
}

fn rec_count(
    polymer_pair: (char, char),
    instructions: &HashMap<(char, char), char>,
    counter: &mut HashMap<char, u64>,
    rem_steps: u64,
    dyn_counter: &mut HashMap<(char, char, u64), HashMap<char, u64>>,
) {
    if rem_steps == 0 {
        *counter.entry(polymer_pair.0).or_insert(0) += 1;
        return;
    }

    if dyn_counter.contains_key(&(polymer_pair.0, polymer_pair.1, rem_steps)) {
        for c in &dyn_counter[&(polymer_pair.0, polymer_pair.1, rem_steps)] {
            *counter.entry(*c.0).or_insert(0) +=
                match dyn_counter[&(polymer_pair.0, polymer_pair.1, rem_steps)].get(&c.0) {
                    Some(x) => *x,
                    None => 0,
                };
        }
    } else {
        let mut c_counter = counter.clone();

        rec_count(
            (polymer_pair.0, instructions[&polymer_pair]),
            instructions,
            counter,
            rem_steps - 1,
            dyn_counter,
        );

        rec_count(
            (instructions[&polymer_pair], polymer_pair.1),
            instructions,
            counter,
            rem_steps - 1,
            dyn_counter,
        );

        for c in counter {
            *c_counter.entry(*c.0).or_insert(*c.1) = *c.1
                - match c_counter.get(&c.0) {
                    Some(x) => *x,
                    None => 0,
                };
        }
        *dyn_counter
            .entry((polymer_pair.0, polymer_pair.1, rem_steps))
            .or_insert(HashMap::new()) = c_counter;
    }
}
