use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day20.txt").unwrap();
    let mut parts = stream.trim().split("\n\n");

    let enhancement_algo_raw = parts.next().unwrap();
    let input_image_lines = parts.next().unwrap().split("\n");

    let mut enhancement_algo: Vec<usize> = Vec::new();
    let mut input_image: Vec<Vec<usize>> = Vec::new();

    for c in enhancement_algo_raw.chars() {
        enhancement_algo.push(if c == '.' { 0 } else { 1 });
    }

    for input_image_line in input_image_lines {
        let mut pixels = Vec::new();
        for c in input_image_line.chars() {
            pixels.push(if c == '.' { 0 } else { 1 });
        }
        input_image.push(pixels);
    }

    let mut part_1 = 0;

    for i in 0..50 {
        input_image = add_padding(input_image, &enhancement_algo, i);
        input_image = calc_image(input_image, &enhancement_algo);
        if i == 1 {
            for row in &input_image {
                for c in row {
                    if *c == 1 {
                        part_1 += 1;
                    }
                }
            }
        }
    }

    let mut lit = 0;

    for row in input_image {
        for c in row {
            if c == 1 {
                lit += 1;
            }
        }
    }

    let part1_sol = part_1.to_string();
    let part2_sol = lit.to_string();

    (part1_sol, part2_sol)
}

fn add_padding(
    input_image: Vec<Vec<usize>>,
    enhancement_algo: &Vec<usize>,
    iter: usize,
) -> Vec<Vec<usize>> {
    let mut pad_input_image: Vec<Vec<usize>> = Vec::new();

    for i in 0..(input_image.len() + 2) {
        let mut pixels: Vec<usize> = Vec::new();
        for j in 0..(input_image[0].len() + 2) {
            if i == 0 || j == 0 || i >= input_image.len() + 1 || j >= input_image[0].len() + 1 {
                pixels.push(if enhancement_algo[0] == 0 {
                    0
                } else {
                    iter % 2
                });
            } else {
                pixels.push(input_image[i - 1][j - 1]);
            }
        }
        pad_input_image.push(pixels);
    }

    pad_input_image
}

fn calc_image(input_image: Vec<Vec<usize>>, enhancement_algo: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut new_input_image = input_image.clone();

    for i in 0..input_image.len() {
        for j in 0..input_image[0].len() {
            let mut shifts = 8;
            let mut sum = 0;
            for k in -1..2 {
                for l in -1..2 {
                    if (i as isize + k) > 0
                        && (i as isize + k) < input_image.len() as isize - 1
                        && (j as isize + l) > 0
                        && (j as isize + l) < input_image[0].len() as isize - 1
                    {
                        sum += input_image[(i as isize + k) as usize][(j as isize + l) as usize]
                            << shifts;
                    } else {
                        sum += input_image[0][0] << shifts;
                    }

                    shifts -= 1;
                }
            }
            new_input_image[i][j] = enhancement_algo[sum];
        }
    }

    new_input_image
}
