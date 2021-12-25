use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day25.txt").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in stream.lines() {
        let mut map_line: Vec<char> = Vec::new();
        for pos in line.chars() {
            map_line.push(pos);
        }
        map.push(map_line);
    }

    let mut count = 1;

    while step(&mut map) != 0 {
        count += 1;
    }

    (count.to_string(), "Sleigh remotely started!".to_string())
}

fn step(map: &mut Vec<Vec<char>>) -> usize {
    let mut changes = 0;
    let mut new_map = map.clone();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '>' {
                let mut next_pos = j + 1;
                if j == map[i].len() - 1 {
                    next_pos = 0;
                }
                if map[i][next_pos] == '.' {
                    changes += 1;
                    new_map[i][j] = '.';
                    new_map[i][next_pos] = '>';
                }
            }
        }
    }
    *map = new_map;
    let mut new_map = map.clone();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'v' {
                let mut next_pos = i + 1;
                if i == map.len() - 1 {
                    next_pos = 0;
                }
                if map[next_pos][j] == '.' {
                    changes += 1;
                    new_map[i][j] = '.';
                    new_map[next_pos][j] = 'v';
                }
            }
        }
    }
    *map = new_map;
    changes
}
