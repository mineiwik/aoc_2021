use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day24.txt").unwrap();
    let instructions = stream.trim().lines();

    let mut stack: Vec<(usize, isize)> = Vec::new();
    let mut block = 0;

    let mut x_add = 0;

    let mut part1 = vec![0; 14];
    let mut part2 = vec![0; 14];

    let mut push = false;

    for instruction in instructions.skip(1) {
        let instruction: Vec<&str> = instruction.split(" ").collect();
        if instruction[0] == "inp" {
            block += 1;
        }

        if instruction[0] == "div" && instruction[1] == "z" && instruction[2] == "1" {
            push = true;
        } else if instruction[0] == "div" && instruction[1] == "z" && instruction[2] == "26" {
            push = false;
        }

        if instruction[0] == "add" && instruction[1] == "x" && instruction[2] != "z" {
            x_add = instruction[2].parse::<isize>().unwrap();
        }

        if instruction[0] == "add"
            && instruction[1] == "y"
            && instruction[2] != "w"
            && instruction[2] != "25"
            && instruction[2] != "1"
        {
            if push {
                stack.push((block, instruction[2].parse::<isize>().unwrap()));
            } else {
                let res = stack.pop().unwrap();
                if (res.1 + x_add) >= 0 {
                    part1[block] = 9;
                    part2[res.0] = 1;
                    part1[res.0] = 9 - (res.1 + x_add);
                    part2[block] = 1 + (res.1 + x_add);
                } else {
                    part1[res.0] = 9;
                    part2[res.0] = (res.1 + x_add).abs() + 1;
                    part1[block] = 9 + (res.1 + x_add);
                    part2[block] = 1;
                }
            }
        }
    }

    let part1_sol = part1.iter().fold(String::new(), |mut acc, num| {
        acc += &num.to_string();
        acc
    });
    let part2_sol = part2.iter().fold(String::new(), |mut acc, num| {
        acc += &num.to_string();
        acc
    });

    (part1_sol, part2_sol)
}
