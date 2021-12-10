use regex::Regex;
use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day10.txt").unwrap();
    let lines: Vec<&str> = stream.trim().lines().collect();

    let faulty_syntax = remove_correct_syntax(&lines);

    let part1_sol = part1(&faulty_syntax);
    let part2_sol = part2(&faulty_syntax);

    (part1_sol, part2_sol)
}

fn part1(faulty_syntax: &Vec<String>) -> String {
    let mut sum: u32 = 0;

    for line in faulty_syntax {
        for a in line.chars() {
            if a == ')' {
                sum += 3;
                break;
            } else if a == ']' {
                sum += 57;
                break;
            } else if a == '}' {
                sum += 1197;
                break;
            } else if a == '>' {
                sum += 25137;
                break;
            }
        }
    }

    return sum.to_string();
}

fn part2(faulty_syntax: &Vec<String>) -> String {
    let mut scores = Vec::new();

    for line in faulty_syntax {
        let mut score: u64 = 0;
        for a in line.chars().rev() {
            if a == '(' {
                score = score * 5 + 1;
                continue;
            } else if a == '[' {
                score = score * 5 + 2;
                continue;
            } else if a == '{' {
                score = score * 5 + 3;
                continue;
            } else if a == '<' {
                score = score * 5 + 4;
                continue;
            } else {
                score = 0;
                break;
            }
        }
        if score > 0 {
            scores.push(score);
        }
    }

    scores.sort();
    return scores[scores.len() / 2].to_string();
}

fn remove_correct_syntax(lines: &Vec<&str>) -> Vec<String> {
    let regex = Regex::new(r"(\(\))|(\{\})|(\[\])|(<>)").unwrap();
    let substitution = "";
    let mut faulty_syntax: Vec<String> = Vec::new();
    for line in lines {
        let mut last_result = String::new();
        let mut result = line.to_string();
        while last_result != result {
            last_result = result.clone();
            result = regex.replace_all(&result, substitution).to_string();
        }
        faulty_syntax.push(result);
    }
    faulty_syntax
}
