use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day08.txt").unwrap();
    let entries: Vec<(&str, &str)> = stream
        .trim()
        .lines()
        .map(|line| {
            let mut entries = line.split(" | ");
            let sig_pat = entries.next().unwrap();
            let dig_out = entries.next().unwrap();
            (sig_pat, dig_out)
        })
        .collect();

    let part1_sol = part1(&entries);
    let part2_sol = part2(&entries);

    (part1_sol, part2_sol)
}

fn part1(entries: &Vec<(&str, &str)>) -> String {
    let mut sum = 0;
    for entry in entries {
        let dig_outs: Vec<&str> = entry.1.split(" ").collect();
        for dig_out in dig_outs {
            if dig_out.len() == 2 || dig_out.len() == 3 || dig_out.len() == 4 || dig_out.len() == 7
            {
                sum += 1;
            }
        }
    }
    return sum.to_string();
}

fn part2(entries: &Vec<(&str, &str)>) -> String {
    let mut sum = 0;
    for entry in entries {
        let mut segments: Vec<HashSet<char>> = Vec::new();
        let mut sig_pats: Vec<&str> = entry.0.split(" ").collect();
        let dig_outs: Vec<&str> = entry.1.split(" ").collect();

        for _i in 0..7 {
            let set: HashSet<char> = HashSet::new();
            segments.push(set);
        }

        sig_pats.sort_by(|a, b| a.len().cmp(&b.len()));
        let last = sig_pats.pop().unwrap();
        sig_pats.insert(3, last);

        let mut sequences: HashMap<usize, HashSet<char>> = HashMap::new();

        for sig_pat in sig_pats {
            let chars = sig_pat.chars();
            let mut char_set: HashSet<char> = HashSet::new();
            for i in chars.clone() {
                char_set.insert(i);
            }
            match sig_pat.len() {
                2 => {
                    sequences.insert(1, sig_pat.chars().collect());
                    for i in chars {
                        segments[2].insert(i);
                        segments[3].insert(i);
                    }
                }
                3 => {
                    sequences.insert(7, sig_pat.chars().collect());
                    for i in chars {
                        if !segments[2].contains(&i) && !segments[3].contains(&i) {
                            segments[1].insert(i);
                        }
                    }
                }
                4 => {
                    sequences.insert(4, sig_pat.chars().collect());
                    for i in chars {
                        if !segments[2].contains(&i) && !segments[3].contains(&i) {
                            segments[0].insert(i);
                            segments[6].insert(i);
                        }
                    }
                }
                5 => {
                    if char_set
                        .intersection(&segments[2])
                        .collect::<HashSet<&char>>()
                        .len()
                        == 1
                        && char_set
                            .intersection(&segments[5])
                            .collect::<HashSet<&char>>()
                            .len()
                            == 1
                    {
                        sequences.insert(5, sig_pat.chars().collect());
                    } else if char_set
                        .intersection(&segments[3])
                        .collect::<HashSet<&char>>()
                        .len()
                        == 1
                        && char_set
                            .intersection(&segments[3])
                            .collect::<HashSet<&char>>()
                            .len()
                            == 1
                    {
                        sequences.insert(2, sig_pat.chars().collect());
                    } else {
                        sequences.insert(3, sig_pat.chars().collect());
                    }
                }
                6 => {
                    if char_set
                        .intersection(&segments[2])
                        .collect::<HashSet<&char>>()
                        .len()
                        == 1
                        && segments[2].len() > 1
                    {
                        sequences.insert(6, sig_pat.chars().collect());
                        segments[2] = segments[2].difference(&char_set).copied().collect();
                        segments[3] = segments[3].difference(&segments[2]).copied().collect();
                    } else if char_set
                        .intersection(&segments[5])
                        .collect::<HashSet<&char>>()
                        .len()
                        == 1
                        && segments[5].len() > 1
                    {
                        sequences.insert(9, sig_pat.chars().collect());
                        segments[5] = segments[5].difference(&char_set).copied().collect();
                        segments[4] = segments[4].difference(&segments[5]).copied().collect();
                    } else {
                        sequences.insert(0, sig_pat.chars().collect());
                        segments[6] = segments[6].difference(&char_set).copied().collect();
                        segments[0] = segments[0].difference(&segments[6]).copied().collect();
                    }
                }
                7 => {
                    sequences.insert(8, sig_pat.chars().collect());
                    for i in chars {
                        if !segments[0].contains(&i)
                            && !segments[1].contains(&i)
                            && !segments[6].contains(&i)
                            && !segments[2].contains(&i)
                            && !segments[3].contains(&i)
                        {
                            segments[4].insert(i);
                            segments[5].insert(i);
                        }
                    }
                }
                _ => continue,
            }
        }

        let mut current_number = 0;

        for (i, dig_out) in dig_outs.iter().rev().enumerate() {
            let actual: HashSet<char> = dig_out.chars().collect();
            for (j, s) in &sequences {
                if actual == *s {
                    current_number += 10usize.pow(i as u32) * j;
                }
            }
        }
        sum += current_number;
    }

    return sum.to_string();
}
