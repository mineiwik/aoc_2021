use std::fs;

struct Submarine {
    // aim was chosen as usize, since my input aim never became negative, albeit this would be possible!
    aim: usize,
    horizontal: usize,
    vertical: usize,
}

impl Submarine {
    fn new() -> Self {
        Self {
            aim: 0,
            horizontal: 0,
            vertical: 0,
        }
    }

    fn forward(&mut self, value: usize) {
        self.horizontal += value;
        self.vertical += self.aim * value;
    }

    fn up(&mut self, value: usize) {
        self.aim -= value;
    }

    fn down(&mut self, value: usize) {
        self.aim += value;
    }

    fn prod(&self) -> usize {
        self.horizontal * self.vertical
    }
}

pub fn solve() -> (String, String) {
    let stream: String = fs::read_to_string("inputs/day02.txt").unwrap();
    let commands: Vec<(&str, usize)> = stream
        .lines()
        .map(|i| {
            let mut a = i.split_whitespace();
            (
                a.next().unwrap(),
                a.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let part1_sol = part1(&commands);
    let part2_sol = part2(&commands);

    (part1_sol, part2_sol)
}

fn part1(commands: &Vec<(&str, usize)>) -> String {
    let mut horizontal = 0;
    let mut vertical = 0;

    for command in commands {
        if command.0 == "up" {
            vertical -= command.1;
        } else if command.0 == "down" {
            vertical += command.1;
        } else if command.0 == "forward" {
            horizontal += command.1;
        }
    }

    return (vertical * horizontal).to_string();
}

fn part2(commands: &Vec<(&str, usize)>) -> String {
    let mut submarine = Submarine::new();
    for command in commands {
        if command.0 == "up" {
            submarine.up(command.1);
        } else if command.0 == "down" {
            submarine.down(command.1);
        } else if command.0 == "forward" {
            submarine.forward(command.1);
        }
    }

    return submarine.prod().to_string();
}
