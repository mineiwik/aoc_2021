mod days;

// use days::{};
use std::env;

fn main() {
    let days: Vec<u8>;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Since no day was selected, all days will be solved!");
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
        let solve: fn() = match day {
            _ => {
                if args.len() >= 2 {
                    println!("\nDay {:02} is not implemented!", day);
                }
                continue;
            }
        };

        println!("\n=== Day {:02} ===", day);
        solve();
    }
}
