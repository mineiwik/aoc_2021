extern crate termion;

mod days;

use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18,
};
use std::env;
use termion::{color, style};

const LATEST_DAY: u8 = 18;

fn main() {
    let days: Vec<u8>;
    let args: Vec<String> = env::args().collect();

    println!(
        "{1}Advent {2}of {1}Code {2}2021{0}",
        style::Reset,
        color::Fg(color::Red),
        color::Fg(color::Green)
    );
    if args.len() < 2 {
        days = vec![LATEST_DAY];
    } else if args[1] == "all" {
        days = (1..=25).collect::<Vec<u8>>();
    } else {
        days = args
            .iter()
            .skip(1)
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
            })
            .collect();
    }

    for day in days {
        let solve: fn() -> (String, String) = match day {
            1 => day01::solve,
            2 => day02::solve,
            3 => day03::solve,
            4 => day04::solve,
            5 => day05::solve,
            6 => day06::solve,
            7 => day07::solve,
            8 => day08::solve,
            9 => day09::solve,
            10 => day10::solve,
            11 => day11::solve,
            12 => day12::solve,
            13 => day13::solve,
            14 => day14::solve,
            15 => day15::solve,
            16 => day16::solve,
            17 => day17::solve,
            18 => day18::solve,
            _ => continue,
        };

        let (part1_sol, part2_sol) = solve();

        println!(
            "{0}{1}\n=== Day {day:02} ==={0}",
            style::Reset,
            color::Fg(color::Green),
            day = day
        );
        println!(
            "{0}{1}### Part 1 ###{0}",
            style::Reset,
            color::Fg(color::Blue)
        );
        println!("{}", part1_sol);
        println!(
            "{0}{1}##############{0}",
            style::Reset,
            color::Fg(color::Blue)
        );
        println!(
            "{0}{1}### Part 2 ###{0}",
            style::Reset,
            color::Fg(color::Blue)
        );
        println!("{}", part2_sol);
        println!(
            "{0}{1}##############{0}",
            style::Reset,
            color::Fg(color::Blue)
        );
        println!(
            "{0}{1}=============={0}",
            style::Reset,
            color::Fg(color::Green)
        );
    }
}
