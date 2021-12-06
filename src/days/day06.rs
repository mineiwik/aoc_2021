use std::fs;

const STATES: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

#[derive(Debug)]
struct Spawner {
    states: Vec<usize>,
}

impl Spawner {
    fn new(lanterns: &Vec<usize>) -> Self {
        let mut states: Vec<usize> = vec![0; STATES.len()];

        for lantern in lanterns {
            states[*lantern] += 1;
        }

        Self { states }
    }

    fn clock_pulse(&mut self) {
        let mut new_states: Vec<usize> = vec![0; STATES.len()];

        for (idx, value) in self.states.iter().enumerate() {
            let next_state = if idx == 0 { 6 } else { idx - 1 };
            if idx == 0 {
                new_states[STATES.len() - 1] += value;
            }
            new_states[next_state] += value;
        }

        self.states = new_states;
    }

    fn count_fishes(&self) -> usize {
        let mut sum = 0;
        for value in &self.states {
            sum += value;
        }
        sum
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day06.txt").unwrap();
    let lanterns: Vec<usize> = stream
        .split(",")
        .map(|lantern| lantern.parse::<usize>().unwrap())
        .collect();

    let part1_sol = part1(&lanterns);
    let part2_sol = part2(&lanterns);

    (part1_sol, part2_sol)
}

fn part1(lanterns: &Vec<usize>) -> String {
    return count_fishes_for(80, lanterns).to_string();
}

fn part2(lanterns: &Vec<usize>) -> String {
    return count_fishes_for(256, lanterns).to_string();
}

fn count_fishes_for(days: usize, lanterns: &Vec<usize>) -> usize {
    let mut spawner = Spawner::new(lanterns);

    for _i in 0..days {
        spawner.clock_pulse();
    }

    spawner.count_fishes()
}
