mod days;

use days::{day_00};
use std::env;

fn main() {
    let days: Vec<u8>;
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No argument was given --> Solving latest day...");
        days = vec![0];
    } else if args[1] == "all" {
        println!("Solving all possible days...");
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
            0 => day_00::solve,
            _ => continue
        };

        let (part1_sol, part2_sol) = solve();

        println!("\n=== Day {:02} ===", day);
        println!("### Part 1 ###");
        println!("{}", part1_sol);
        println!("##############");
        println!("### Part 2 ###");
        println!("{}", part2_sol);
        println!("##############");
        println!("==============");
    }
}
