use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day17.txt").unwrap();

    let input = stream.replace("target area: ", "");
    let mut coord = input.split(", ");

    let x_range = coord.next().unwrap().replace("x=", "");
    let mut x_range = x_range.split("..");
    let x_range = (
        x_range.next().unwrap().parse::<isize>().unwrap(),
        x_range.next().unwrap().parse::<isize>().unwrap(),
    );
    let y_range = coord.next().unwrap().replace("y=", "");
    let mut y_range = y_range.split("..");
    let y_range = (
        y_range.next().unwrap().parse::<isize>().unwrap(),
        y_range.next().unwrap().parse::<isize>().unwrap(),
    );

    let part1_sol = find_highest_y(y_range).to_string();
    let part2_sol = find_all_trajectories(x_range, y_range).to_string();

    (part1_sol, part2_sol)
}

fn find_highest_y(y_range: (isize, isize)) -> isize {
    (y_range.0.abs() - 1) * y_range.0.abs() / 2
}

fn find_all_trajectories(x_range: (isize, isize), y_range: (isize, isize)) -> isize {
    let mut count = 0;
    let low_x = ((2 * x_range.0) as f64).sqrt() as isize;
    let high_x = x_range.1;
    let low_y = y_range.0;
    let high_y = y_range.0.abs();

    for cur_x in low_x..=high_x {
        for cur_y in low_y..=high_y {
            if valid_trajectory((cur_x, cur_y), x_range, y_range) {
                count += 1;
            }
        }
    }
    count
}

fn valid_trajectory(vel: (isize, isize), x_range: (isize, isize), y_range: (isize, isize)) -> bool {
    let mut pos: (isize, isize) = (0, 0);
    let mut cur_x = vel.0;
    let mut cur_y = vel.1;
    loop {
        pos.1 += cur_y;
        pos.0 += cur_x;
        if cur_x > 0 {
            cur_x -= 1;
        } else if cur_x < 0 {
            cur_x += 1;
        }
        cur_y -= 1;
        if pos.0 >= x_range.0 && pos.0 <= x_range.1 && pos.1 >= y_range.0 && pos.1 <= y_range.1 {
            return true;
        }
        if pos.0 > x_range.1 || pos.1 < y_range.0 {
            return false;
        }
    }
}
