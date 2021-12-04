use std::collections::HashMap;
use std::fs;

const WIDTH: u8 = 5;
const HEIGHT: u8 = 5;

#[derive(Debug)]
struct Bingo {
    cards: Vec<Card>,
}

impl Bingo {
    fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    fn draw_num(&mut self, num: u8) -> (i32, usize) {
        let mut sum: Vec<i32> = Vec::new();
        let mut del_idx: Vec<usize> = Vec::new();

        for card in &mut self.cards {
            let loc = match card.numbers.get(&num) {
                Some(i) => i,
                None => {
                    del_idx.push(0);
                    continue;
                }
            };

            *card.marked.get_mut(&num).unwrap() = true;
            card.rows[loc.0] -= 1;
            card.columns[loc.1] -= 1;

            if card.rows[loc.0] == 0 || card.columns[loc.1] == 0 {
                sum.push(
                    card.marked
                        .iter()
                        .filter(|&(_, v)| !*v)
                        .map(|(&k, _)| k as i32)
                        .sum(),
                );
                del_idx.push(1);
            } else {
                del_idx.push(0);
            }
        }

        if sum.len() > 0 {
            let mut index = 0;
            self.cards.retain(|_| {
                index += 1;
                del_idx[index - 1] == 0
            });
            return (sum[0] * num as i32, self.cards.len());
        }

        (-1, self.cards.len())
    }
}

#[derive(Debug)]
struct Card {
    numbers: HashMap<u8, (usize, usize)>,
    marked: HashMap<u8, bool>,
    rows: Vec<u8>,
    columns: Vec<u8>,
}

impl Card {
    fn new(card: &str) -> Self {
        let mut numbers: HashMap<u8, (usize, usize)> = HashMap::new();
        let mut marked: HashMap<u8, bool> = HashMap::new();
        let rows: Vec<u8> = vec![WIDTH; HEIGHT as usize];
        let columns: Vec<u8> = vec![HEIGHT; WIDTH as usize];

        for (i, row) in card.split("\n").enumerate() {
            for (j, num) in row.split_whitespace().enumerate() {
                numbers.insert(num.parse::<u8>().unwrap(), (i, j));
                marked.insert(num.parse::<u8>().unwrap(), false);
            }
        }

        Self {
            numbers,
            marked,
            rows,
            columns,
        }
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day04.txt").unwrap();
    let lines: Vec<&str> = stream.split("\n\n").collect();

    let part1_sol = part1(&lines);
    let part2_sol = part2(&lines);

    (part1_sol, part2_sol)
}

fn part1(lines: &Vec<&str>) -> String {
    let drawn: Vec<u8> = lines[0]
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect();
    let cards: Vec<Card> = lines.iter().skip(1).map(|card| Card::new(card)).collect();

    let mut res: (i32, usize) = (-1, 0);

    let mut bingo = Bingo::new(cards);

    for draw in drawn {
        res = bingo.draw_num(draw);
        if res.0 != -1 {
            break;
        }
    }

    return res.0.to_string();
}

fn part2(lines: &Vec<&str>) -> String {
    let drawn: Vec<u8> = lines[0]
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect();
    let cards: Vec<Card> = lines.iter().skip(1).map(|card| Card::new(card)).collect();

    let mut res: (i32, usize) = (-1, 0);

    let mut bingo = Bingo::new(cards);

    for draw in drawn {
        res = bingo.draw_num(draw);
        if res.0 != -1 && res.1 == 0 {
            break;
        }
    }

    return (res.0).to_string();
}
