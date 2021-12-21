use std::collections::HashMap;
use std::fs;

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day21.txt").unwrap();
    let mut input = stream.trim().split("\n");

    let player_1_pos = input.next().unwrap().split("starting position: ");
    let player_1_pos = player_1_pos
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let player_2_pos = input.next().unwrap().split("starting position: ");
    let player_2_pos = player_2_pos
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let part1_sol = part1(player_1_pos, player_2_pos);
    let part2_sol = part2(player_1_pos, player_2_pos);

    (part1_sol, part2_sol)
}

fn part1(player_1_pos: usize, player_2_pos: usize) -> String {
    let mut players: Vec<(usize, usize)> = Vec::new();

    players.push((player_1_pos - 1, 0));
    players.push((player_2_pos - 1, 0));

    deterministic_game(players).to_string()
}

fn part2(player_1_pos: usize, player_2_pos: usize) -> String {
    let res = quantum_game(
        player_1_pos - 1,
        0,
        player_2_pos - 1,
        0,
        &mut HashMap::new(),
    );
    std::cmp::max(res.0, res.1).to_string()
}

fn deterministic_game(mut players: Vec<(usize, usize)>) -> usize {
    let mut die_value = 0;
    let mut die_count = 0;
    let mut player_index = 0;

    while players[0].1 < 1000 && players[1].1 < 1000 {
        for _i in 0..3 {
            deterministic_die(&mut die_count, &mut die_value);
            players[player_index].0 = (players[player_index].0 + die_value) % 10;
        }
        players[player_index].1 += players[player_index].0 + 1;

        player_index = (player_index + 1) % 2;
    }

    std::cmp::min(players[0].1, players[1].1) * die_count
}

fn quantum_game(
    current_player_pos: usize,
    current_player_score: usize,
    other_player_pos: usize,
    other_player_score: usize,
    mem: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
) -> (usize, usize) {
    if current_player_score >= 21 {
        return (1, 0);
    }

    if other_player_score >= 21 {
        return (0, 1);
    }

    if mem.contains_key(&(
        current_player_pos,
        current_player_score,
        other_player_pos,
        other_player_score,
    )) {
        return mem[&(
            current_player_pos,
            current_player_score,
            other_player_pos,
            other_player_score,
        )];
    }

    let mut wins = (0, 0);

    for roll_1 in 1..=3 {
        for roll_2 in 1..=3 {
            for roll_3 in 1..=3 {
                let forward = roll_1 + roll_2 + roll_3;
                let new_current_player_pos = (current_player_pos + forward) % 10;
                let cur_wins = quantum_game(
                    other_player_pos,
                    other_player_score,
                    new_current_player_pos,
                    current_player_score + new_current_player_pos + 1,
                    mem,
                );
                wins.0 += cur_wins.1;
                wins.1 += cur_wins.0;
            }
        }
    }
    mem.insert(
        (
            current_player_pos,
            current_player_score,
            other_player_pos,
            other_player_score,
        ),
        wins,
    );
    wins
}

fn deterministic_die(count: &mut usize, value: &mut usize) {
    *count += 1;
    *value = if *value == 100 { 1 } else { *value + 1 };
}
