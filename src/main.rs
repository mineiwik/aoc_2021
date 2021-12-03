extern crate termion;

mod days;

use days::{day01, day02, day03};
use std::env;
use termion::{color, style};

const LATEST_DAY: u8 = 3;

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
