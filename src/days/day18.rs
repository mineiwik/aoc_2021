use std::fs;

#[derive(Debug)]
enum Element {
    Pair(Pair),
    Number(usize),
}

#[derive(Debug)]
struct Pair {
    left: Box<Element>,
    right: Box<Element>,
}

impl Pair {
    fn new(left: Element, right: Element) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn find_first_number(&mut self, num: usize) {
        if let Element::Number(x) = &mut *self.left {
            self.left = Box::new(Element::Number(*x + num));
            return;
        } else if let Element::Pair(x) = &mut *self.left {
            x.find_first_number(num);
        } else if let Element::Number(x) = &mut *self.right {
            self.right = Box::new(Element::Number(*x + num));
            return;
        } else if let Element::Pair(x) = &mut *self.right {
            x.find_first_number(num);
        }
    }

    fn find_first_left_number(&mut self, num: usize) {
        if let Element::Number(x) = &mut *self.right {
            self.right = Box::new(Element::Number(*x + num));
            return;
        } else if let Element::Pair(x) = &mut *self.right {
            x.find_first_left_number(num);
        } else if let Element::Number(x) = &mut *self.left {
            self.left = Box::new(Element::Number(*x + num));
            return;
        } else if let Element::Pair(x) = &mut *self.left {
            x.find_first_left_number(num);
        }
    }

    fn find_first_explodable(&mut self, depth: usize) -> (usize, usize, bool, bool) {
        if depth == 4 {
            let mut ret_left = 0;
            let mut ret_right = 0;
            if let Element::Number(x) = &mut *self.left {
                ret_left = *x;
            }
            if let Element::Number(x) = &mut *self.right {
                ret_right = *x;
            }
            return (ret_left, ret_right, true, true);
        }

        if let Element::Pair(x) = &mut *self.left {
            let res = x.find_first_explodable(depth + 1);
            if res.2 {
                self.left = Box::new(Element::Number(0));
                if let Element::Number(x) = &mut *self.right {
                    self.right = Box::new(Element::Number(*x + res.1));
                } else if let Element::Pair(x) = &mut *self.right {
                    x.find_first_number(res.1);
                }
                return (res.0, 0, false, true);
            } else if res.3 {
                if res.1 == 0 {
                    return (res.0, 0, false, true);
                }
                if let Element::Number(x) = &mut *self.right {
                    self.right = Box::new(Element::Number(*x + res.1));
                } else if let Element::Pair(x) = &mut *self.right {
                    x.find_first_number(res.1);
                }

                return (res.0, 0, false, true);
            }
        }

        if let Element::Pair(x) = &mut *self.right {
            let res = x.find_first_explodable(depth + 1);
            if res.2 {
                if let Element::Number(x) = &mut *self.left {
                    self.left = Box::new(Element::Number(*x + res.0));
                }
                self.right = Box::new(Element::Number(0));
                return (0, res.1, false, true);
            } else if res.3 {
                if res.0 == 0 {
                    return (0, res.1, false, true);
                }
                if let Element::Number(x) = &mut *self.left {
                    self.left = Box::new(Element::Number(*x + res.0));
                    return (0, 0, false, true);
                } else if let Element::Pair(x) = &mut *self.left {
                    x.find_first_left_number(res.0);
                    return (0, 0, false, true);
                }
                return (res.0, 0, false, true);
            }
        }

        (0, 0, false, false)
    }

    fn find_first_splittable(&mut self) -> bool {
        match &mut *self.left {
            Element::Pair(x) => {
                if x.find_first_splittable() {
                    return true;
                }
            }
            Element::Number(x) => {
                if *x > 9 {
                    self.left = Box::new(Element::Pair(Pair::new(
                        Element::Number(*x / 2),
                        Element::Number(if *x % 2 == 0 { *x / 2 } else { *x / 2 + 1 }),
                    )));
                    return true;
                }
            }
        }

        match &mut *self.right {
            Element::Pair(x) => return x.find_first_splittable(),
            Element::Number(x) => {
                if *x > 9 {
                    self.right = Box::new(Element::Pair(Pair::new(
                        Element::Number(*x / 2),
                        Element::Number(if *x % 2 == 0 { *x / 2 } else { *x / 2 + 1 }),
                    )));
                    return true;
                }
                return false;
            }
        }
    }

    fn reduce(&mut self) -> bool {
        if self.find_first_explodable(0).3 {
            return true;
        }
        if self.find_first_splittable() {
            return true;
        }
        false
    }

    fn add(mut self, mut rhs: Pair) -> Self {
        loop {
            if !self.reduce() {
                break;
            }
        }
        loop {
            if !rhs.reduce() {
                break;
            }
        }
        Pair::new(Element::Pair(self), Element::Pair(rhs))
    }

    fn magnitude(&mut self) -> usize {
        let mut mag = 0;
        if let Element::Number(x) = *self.left {
            mag += 3 * x;
        } else if let Element::Pair(x) = &mut *self.left {
            mag += 3 * x.magnitude();
        }

        if let Element::Number(x) = *self.right {
            mag += 2 * x;
        } else if let Element::Pair(x) = &mut *self.right {
            mag += 2 * x.magnitude();
        }

        mag
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day18.txt").unwrap();
    let lines = stream.trim().lines();

    let part1_sol = part1(lines.clone());
    let part2_sol = part2(lines.clone());

    (part1_sol, part2_sol)
}

fn part1(lines: std::str::Lines) -> String {
    let mut pairs: Vec<Pair> = Vec::new();

    let mut first = false;

    let mut sum = Pair::new(Element::Number(0), Element::Number(0));

    for line in lines {
        if !first {
            sum = parse_numbers(line);
            first = true;
            continue;
        }
        pairs.push(parse_numbers(line));
    }

    for pair in pairs {
        sum = sum.add(pair);
    }

    loop {
        if !sum.reduce() {
            break;
        }
    }

    return sum.magnitude().to_string();
}

fn part2(lines: std::str::Lines) -> String {
    let mut max_mag: usize = 0;
    for i in lines.clone() {
        for j in lines.clone() {
            if i == j {
                continue;
            }
            let a = parse_numbers(i);
            let b = parse_numbers(j);
            let mut sum = a.add(b);
            loop {
                if !sum.reduce() {
                    break;
                }
            }
            if sum.magnitude() > max_mag {
                max_mag = sum.magnitude();
            }
        }
    }

    return max_mag.to_string();
}

fn parse_numbers(line: &str) -> Pair {
    let mut pair: Pair = Pair {
        left: Box::new(Element::Number(0)),
        right: Box::new(Element::Number(0)),
    };
    let mut line = line.chars();
    let next_c = line.next().unwrap();
    if next_c == '[' {
        pair = parse_pair(&mut line);
    }

    // STRING HAS BEEN SUCCESSFULLY PARSED!
    pair
}

fn parse_pair(line: &mut std::str::Chars) -> Pair {
    let mut next_c = line.next().unwrap();
    let left: Element;
    let right: Element;
    if next_c == '[' {
        left = Element::Pair(parse_pair(line));
        line.next().unwrap();
    } else {
        let mut num = String::new();
        while next_c != ',' {
            if next_c.is_digit(10) {
                // Well, we've FINALLY found a number!
                num.push(next_c);
            }
            next_c = line.next().unwrap();
        }
        // First part of pair has been parsed!
        // We know that it MUST be a literal (number)
        left = Element::Number(num.parse::<usize>().unwrap());
    }

    next_c = line.next().unwrap();

    if next_c == '[' {
        right = Element::Pair(parse_pair(line));
        line.next().unwrap();
    } else {
        let mut num = String::new();
        while next_c != ']' {
            if next_c.is_digit(10) {
                // Well, we've FINALLY found a number!
                num.push(next_c);
            }
            next_c = line.next().unwrap();
        }
        // Second part of pair has been parsed!
        // We know that it MUST be a literal (number)
        right = Element::Number(num.parse::<usize>().unwrap());
    }

    Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}
