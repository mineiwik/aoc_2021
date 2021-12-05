use std::collections::HashMap;
use std::{cmp, fs};

#[derive(Debug)]
struct Vent {
    start: (isize, isize),
    end: (isize, isize),
    diagonal: bool,
}

impl Vent {
    fn new(vent: ((isize, isize), (isize, isize))) -> Self {
        let start = vent.0;
        let end = vent.1;
        let diagonal = if start.0 == end.0 || start.1 == end.1 {
            false
        } else {
            true
        };

        Self {
            start,
            end,
            diagonal,
        }
    }
}

#[derive(Debug)]
struct Diagram {
    vents: Vec<Vent>,
    coordinates: HashMap<(isize, isize), isize>,
}

impl Diagram {
    fn new(vents: Vec<Vent>) -> Self {
        let coordinates = HashMap::new();
        Self { vents, coordinates }
    }

    fn calc_clouds(&mut self, diagonal: bool) {
        for vent in &self.vents {
            if diagonal || !diagonal && !vent.diagonal {
                let horizontal_steps = vent.end.0 - vent.start.0;
                let vertical_steps = vent.end.1 - vent.start.1;

                let horizontal_unit = if horizontal_steps == 0 {
                    0
                } else {
                    horizontal_steps.abs() / horizontal_steps
                };
                let vertical_unit = if vertical_steps == 0 {
                    0
                } else {
                    vertical_steps.abs() / vertical_steps
                };

                for i in 0..=cmp::max(horizontal_steps.abs(), vertical_steps.abs()) {
                    *self
                        .coordinates
                        .entry((
                            vent.start.0 + i * horizontal_unit,
                            vent.start.1 + i * vertical_unit,
                        ))
                        .or_insert(0) += 1;
                }
            }
        }
    }

    fn get_overlaps(&self) -> isize {
        let mut sum = 0;
        for (_, v) in &self.coordinates {
            if *v >= 2 {
                sum += 1;
            }
        }
        sum
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day05.txt").unwrap();
    let lines: Vec<((isize, isize), (isize, isize))> = stream
        .lines()
        .map(|line| {
            let mut elem = line.split(" -> ");
            let start: Vec<isize> = elem
                .next()
                .unwrap()
                .split(",")
                .map(|i| i.parse::<isize>().unwrap())
                .collect();
            let end: Vec<isize> = elem
                .next()
                .unwrap()
                .split(",")
                .map(|i| i.parse::<isize>().unwrap())
                .collect();
            ((start[0], start[1]), (end[0], end[1]))
        })
        .collect();

    let part1_sol = part1(&lines);
    let part2_sol = part2(&lines);

    (part1_sol, part2_sol)
}

fn part1(lines: &Vec<((isize, isize), (isize, isize))>) -> String {
    let mut vents: Vec<Vent> = Vec::new();
    for line in lines {
        vents.push(Vent::new(*line));
    }

    let mut diagram = Diagram::new(vents);

    diagram.calc_clouds(false);

    return diagram.get_overlaps().to_string();
}

fn part2(lines: &Vec<((isize, isize), (isize, isize))>) -> String {
    let mut vents: Vec<Vent> = Vec::new();
    for line in lines {
        vents.push(Vent::new(*line));
    }

    let mut diagram = Diagram::new(vents);

    diagram.calc_clouds(true);

    return diagram.get_overlaps().to_string();
}
