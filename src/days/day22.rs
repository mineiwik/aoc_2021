use std::fs;

#[derive(Debug)]
struct Step {
    cuboid: Cuboid,
    on: bool,
}

#[derive(Debug, Clone)]
struct Cuboid {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Cuboid {
    fn overlaps(&self, other: &Cuboid) -> bool {
        if self.x_min <= other.x_max
            && self.x_max >= other.x_min
            && self.y_min <= other.y_max
            && self.y_max >= other.y_min
            && self.z_min <= other.z_max
            && self.z_max >= other.z_min
        {
            return true;
        }
        false
    }

    fn get_new_cuboids(&mut self, other: &Cuboid) -> Vec<Cuboid> {
        let mut new_cuboids: Vec<Cuboid> = Vec::new();
        if self.overlaps(other) {
            if self.x_min < other.x_min {
                new_cuboids.push(Cuboid {
                    x_min: self.x_min,
                    x_max: other.x_min - 1,
                    y_min: self.y_min,
                    y_max: self.y_max,
                    z_min: self.z_min,
                    z_max: self.z_max,
                });
                self.x_min = other.x_min;
            }
            if self.x_max > other.x_max {
                new_cuboids.push(Cuboid {
                    x_min: other.x_max + 1,
                    x_max: self.x_max,
                    y_min: self.y_min,
                    y_max: self.y_max,
                    z_min: self.z_min,
                    z_max: self.z_max,
                });
                self.x_max = other.x_max;
            }
            if self.y_min < other.y_min {
                new_cuboids.push(Cuboid {
                    x_min: self.x_min,
                    x_max: self.x_max,
                    y_min: self.y_min,
                    y_max: other.y_min - 1,
                    z_min: self.z_min,
                    z_max: self.z_max,
                });
                self.y_min = other.y_min;
            }
            if self.y_max > other.y_max {
                new_cuboids.push(Cuboid {
                    x_min: self.x_min,
                    x_max: self.x_max,
                    y_min: other.y_max + 1,
                    y_max: self.y_max,
                    z_min: self.z_min,
                    z_max: self.z_max,
                });
                self.y_max = other.y_max;
            }
            if self.z_min < other.z_min {
                new_cuboids.push(Cuboid {
                    x_min: self.x_min,
                    x_max: self.x_max,
                    y_min: self.y_min,
                    y_max: self.y_max,
                    z_min: self.z_min,
                    z_max: other.z_min - 1,
                });
                self.z_min = other.z_min;
            }
            if self.z_max > other.z_max {
                new_cuboids.push(Cuboid {
                    x_min: self.x_min,
                    x_max: self.x_max,
                    y_min: self.y_min,
                    y_max: self.y_max,
                    z_min: other.z_max + 1,
                    z_max: self.z_max,
                });
                self.z_max = other.z_max;
            }
        } else {
            new_cuboids.push(self.clone());
        }
        new_cuboids
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day22.txt").unwrap();
    let steps: Vec<Step> = stream
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.split(" ");
            let on = if parts.next().unwrap() == "on" {
                true
            } else {
                false
            };
            let mut ranges = parts.next().unwrap().split(",");
            let x_range = ranges.next().unwrap().replace("x=", "");
            let mut x_range = x_range.split("..");
            let x_min = x_range.next().unwrap().parse::<isize>().unwrap();
            let x_max = x_range.next().unwrap().parse::<isize>().unwrap();

            let y_range = ranges.next().unwrap().replace("y=", "");
            let mut y_range = y_range.split("..");
            let y_min = y_range.next().unwrap().parse::<isize>().unwrap();
            let y_max = y_range.next().unwrap().parse::<isize>().unwrap();

            let z_range = ranges.next().unwrap().replace("z=", "");
            let mut z_range = z_range.split("..");
            let z_min = z_range.next().unwrap().parse::<isize>().unwrap();
            let z_max = z_range.next().unwrap().parse::<isize>().unwrap();

            Step {
                cuboid: Cuboid {
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    z_min,
                    z_max,
                },
                on,
            }
        })
        .collect();

    let cuboids = simulate(&steps);

    let part1_sol = part1(&cuboids);
    let part2_sol = part2(&cuboids);

    (part1_sol, part2_sol)
}

fn simulate(steps: &Vec<Step>) -> Vec<Cuboid> {
    steps.iter().fold(Vec::<Cuboid>::new(), |mut acc, step| {
        let mut current_cuboids = Vec::new();
        for cuboid in acc.iter_mut() {
            current_cuboids.append(&mut cuboid.get_new_cuboids(&step.cuboid));
        }
        if step.on {
            current_cuboids.push(step.cuboid.clone());
        }
        current_cuboids
    })
}

fn part1(cuboids: &Vec<Cuboid>) -> String {
    let sum: isize = cuboids
        .iter()
        .filter(|a| {
            a.x_min >= -50
                && a.x_max <= 50
                && a.y_min >= -50
                && a.y_max <= 50
                && a.z_min >= -50
                && a.z_max <= 50
        })
        .map(|a| (a.x_max - a.x_min + 1) * (a.y_max - a.y_min + 1) * (a.z_max - a.z_min + 1))
        .sum();
    sum.to_string()
}

fn part2(cuboids: &Vec<Cuboid>) -> String {
    let sum: isize = cuboids
        .iter()
        .map(|a| (a.x_max - a.x_min + 1) * (a.y_max - a.y_min + 1) * (a.z_max - a.z_min + 1))
        .sum();
    sum.to_string()
}
