use std::collections::HashSet;
use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day13.txt").unwrap();
    let lines: Vec<&str> = stream.trim().split("\n\n").collect();

    let dots: HashSet<(u32, u32)> = lines[0]
        .trim()
        .lines()
        .map(|line| {
            let mut c_line = line.split(",");
            (
                c_line.next().unwrap().parse::<u32>().unwrap(),
                c_line.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let instructions: Vec<(u32, u32)> = lines[1]
        .trim()
        .lines()
        .map(|line| {
            let c_line = line.replace("fold along ", "");
            let mut c_line = c_line.split("=");
            let loc = c_line.next().unwrap();
            let coord = c_line.next().unwrap().parse::<u32>().unwrap();
            let x = if loc == "x" { coord } else { 0 };
            let y = if loc == "y" { coord } else { 0 };
            (x, y)
        })
        .collect();

    let part1_sol = part1(&dots, &instructions);
    let part2_sol = part2(&dots, &instructions);

    (part1_sol, part2_sol)
}

fn part1(dots: &HashSet<(u32, u32)>, instructions: &Vec<(u32, u32)>) -> String {
    let mut dots = dots.clone();
    fold(&mut dots, instructions[0]);

    return dots.len().to_string();
}

fn part2(dots: &HashSet<(u32, u32)>, instructions: &Vec<(u32, u32)>) -> String {
    let mut dots = dots.clone();
    for instruction in instructions {
        fold(&mut dots, *instruction);
    }

    let max_x = dots.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0 as usize;
    let max_y = dots.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 as usize;

    let mut render: Vec<Vec<char>> = Vec::new();
    for i in 0..=max_y {
        render.push(Vec::new());
        for _j in 0..=max_x {
            render[i].push(' ');
        }
    }

    for dot in dots.clone() {
        render[dot.1 as usize][dot.0 as usize] = '#';
    }

    let mut res = String::new();

    for line in render {
        for c in line {
            res.push(c);
        }
        res += "\n";
    }

    return res;
}

fn fold(dots: &mut HashSet<(u32, u32)>, instruction: (u32, u32)) {
    for dot in dots.clone().iter() {
        if instruction.0 > 0 {
            // x fold
            if dot.0 > instruction.0 {
                dots.insert((dot.0 - 2 * (dot.0 - instruction.0), dot.1));
                dots.remove(&dot);
            }
        } else {
            //y fold
            if dot.1 > instruction.1 {
                dots.insert((dot.0, dot.1 - 2 * (dot.1 - instruction.1)));
                dots.remove(&dot);
            }
        }
    }
}
