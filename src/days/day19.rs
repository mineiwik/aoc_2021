use std::collections::HashMap;
use std::fs;

const ROTATIONS: [(usize, usize, usize, isize, isize, isize); 24] = [
    (0, 1, 2, 1, 1, 1),
    (2, 0, 1, 1, 1, 1),
    (1, 2, 0, 1, 1, 1),
    (0, 1, 2, -1, -1, 1),
    (0, 1, 2, -1, 1, -1),
    (0, 1, 2, 1, -1, -1),
    (2, 0, 1, -1, -1, 1),
    (2, 0, 1, -1, 1, -1),
    (2, 0, 1, 1, -1, -1),
    (1, 2, 0, -1, -1, 1),
    (1, 2, 0, -1, 1, -1),
    (1, 2, 0, 1, -1, -1),
    (0, 2, 1, 1, 1, -1),
    (0, 2, 1, 1, -1, 1),
    (0, 2, 1, -1, -1, -1),
    (0, 2, 1, -1, 1, 1),
    (2, 1, 0, 1, -1, 1),
    (2, 1, 0, -1, -1, -1),
    (2, 1, 0, 1, 1, -1),
    (2, 1, 0, -1, 1, 1),
    (1, 0, 2, 1, -1, 1),
    (1, 0, 2, -1, -1, -1),
    (1, 0, 2, 1, 1, -1),
    (1, 0, 2, -1, 1, 1),
];

#[derive(Debug)]
struct Beacon {
    pos: Vec<isize>,
}

#[derive(Debug)]
struct Scanner {
    x_abs: isize,
    y_abs: isize,
    z_abs: isize,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new(beacons: Vec<Beacon>) -> Self {
        Self {
            x_abs: 0,
            y_abs: 0,
            z_abs: 0,
            beacons,
        }
    }
}

impl Beacon {
    fn new(x_rel: isize, y_rel: isize, z_rel: isize) -> Self {
        Self {
            pos: vec![x_rel, y_rel, z_rel],
        }
    }

    fn eq(
        &self,
        rhs: &Beacon,
        rot: (usize, usize, usize, isize, isize, isize),
        x: isize,
        y: isize,
        z: isize,
    ) -> bool {
        if self.pos[0] == x + rot.3 * rhs.pos[rot.0]
            && self.pos[1] == y + rot.4 * rhs.pos[rot.1]
            && self.pos[2] == z + rot.5 * rhs.pos[rot.2]
        {
            return true;
        }
        false
    }
}

pub fn solve() -> (String, String) {
    let stream = fs::read_to_string("inputs/day19.txt").unwrap();
    let scanners_input = stream.trim().split("\n\n");

    let mut scanners: Vec<Scanner> = Vec::new();

    for scanner in scanners_input {
        let beacons_input = scanner.lines().skip(1);
        let mut beacons: Vec<Beacon> = Vec::new();
        for beacon in beacons_input {
            let mut pos = beacon.split(",");
            let rel_x = pos.next().unwrap().parse::<isize>().unwrap();
            let rel_y = pos.next().unwrap().parse::<isize>().unwrap();
            let rel_z = pos.next().unwrap().parse::<isize>().unwrap();
            beacons.push(Beacon::new(rel_x, rel_y, rel_z));
        }

        scanners.push(Scanner::new(beacons));
    }

    let mut sat_dist: Vec<(isize, isize, isize)> = Vec::new();

    loop {
        let mut drop: usize = 0;
        let mut new_beacons: Vec<Beacon> = Vec::new();
        for i in 1..scanners.len() {
            let res = find_same_beacons(&scanners[0], &scanners[i]);
            if res.0 {
                sat_dist.push((res.7, res.8, res.9));
                drop = i;
                for new_beacon in &scanners[i].beacons {
                    let mut ok = true;
                    for beacon in &scanners[0].beacons {
                        if beacon.eq(
                            new_beacon,
                            (res.1, res.2, res.3, res.4, res.5, res.6),
                            res.7,
                            res.8,
                            res.9,
                        ) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        new_beacons.push(Beacon::new(
                            res.7 + res.4 * new_beacon.pos[res.1],
                            res.8 + res.5 * new_beacon.pos[res.2],
                            res.9 + res.6 * new_beacon.pos[res.3],
                        ));
                    }
                }
                break;
            }
        }
        scanners.remove(drop);
        scanners[0].beacons.append(&mut new_beacons);

        if scanners.len() == 1 {
            break;
        }
    }

    let mut max = 0;

    for sat_d in &sat_dist {
        for bla in &sat_dist {
            let new_man =
                (sat_d.0 - bla.0).abs() + (sat_d.1 - bla.1).abs() + (sat_d.2 - bla.2).abs();
            if new_man > max {
                max = new_man;
            }
        }
    }

    let part1_sol = scanners[0].beacons.len().to_string();
    let part2_sol = max.to_string();

    (part1_sol, part2_sol)
}

fn find_same_beacons(
    prev_scanner: &Scanner,
    scanner: &Scanner,
) -> (
    bool,
    usize,
    usize,
    usize,
    isize,
    isize,
    isize,
    isize,
    isize,
    isize,
) {
    let mut rots = HashMap::new();
    for beacon_outer in &scanner.beacons {
        for beacon_inner in &scanner.beacons {
            let mut success = false;
            if beacon_inner.pos[0] == beacon_outer.pos[0]
                && beacon_inner.pos[1] == beacon_outer.pos[1]
                && beacon_inner.pos[2] == beacon_outer.pos[2]
            {
                continue;
            }
            let dist = calc_dist(&beacon_outer, &beacon_inner);
            for prev_beacon_outer in &prev_scanner.beacons {
                for prev_beacon_inner in &prev_scanner.beacons {
                    if prev_beacon_inner.pos[0] == prev_beacon_outer.pos[0]
                        && prev_beacon_inner.pos[1] == prev_beacon_outer.pos[1]
                        && prev_beacon_inner.pos[2] == prev_beacon_outer.pos[2]
                    {
                        continue;
                    }
                    let dist_inner = calc_dist(&prev_beacon_outer, &prev_beacon_inner);
                    let is_same = get_rot(&dist, &dist_inner);
                    if is_same.0 {
                        success = true;
                        *rots.entry(is_same).or_insert(0) += 1;
                        if *rots.get(&is_same).unwrap() >= 12 {
                            return (
                                true,
                                is_same.1,
                                is_same.2,
                                is_same.3,
                                is_same.4,
                                is_same.5,
                                is_same.6,
                                prev_beacon_outer.pos[0] - is_same.4 * beacon_outer.pos[is_same.1],
                                prev_beacon_outer.pos[1] - is_same.5 * beacon_outer.pos[is_same.2],
                                prev_beacon_outer.pos[2] - is_same.6 * beacon_outer.pos[is_same.3],
                            );
                        }
                        break;
                    }
                    if success {
                        break;
                    }
                }
                if success {
                    break;
                }
            }
        }
    }
    (false, 0, 0, 0, 0, 0, 0, 0, 0, 0)
}

fn get_rot(lhs: &Vec<isize>, rhs: &Vec<isize>) -> (bool, usize, usize, usize, isize, isize, isize) {
    for rotation in ROTATIONS {
        if rotation.3 * lhs[rotation.0] == rhs[0]
            && rotation.4 * lhs[rotation.1] == rhs[1]
            && rotation.5 * lhs[rotation.2] == rhs[2]
        {
            return (
                true, rotation.0, rotation.1, rotation.2, rotation.3, rotation.4, rotation.5,
            );
        }
    }
    (false, 0, 0, 0, 0, 0, 0)
}

fn calc_dist(lhs: &Beacon, rhs: &Beacon) -> Vec<isize> {
    let dist = vec![
        rhs.pos[0] - lhs.pos[0],
        rhs.pos[1] - lhs.pos[1],
        rhs.pos[2] - lhs.pos[2],
    ];
    dist
}
