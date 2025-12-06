mod days;
mod utils;

use clap::Parser;
use days::Solution;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Advent of Code 2015 Solutions", long_about = None)]
struct Args {
    /// Day number (1-25)
    #[arg(short, long)]
    day: u8,

    /// Run part 1, part 2, or both (default: both)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Use example input instead of real input
    #[arg(short, long)]
    example: bool,

    /// Run benchmarks
    #[arg(short, long)]
    bench: bool,
}

fn get_solution(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(days::day01::Day01)),
        2 => Some(Box::new(days::day02::Day02)),
        3 => Some(Box::new(days::day03::Day03)),
        4 => Some(Box::new(days::day04::Day04)),
        _ => None,
    }
}

fn main() {
    let args = Args::parse();

    let solution = get_solution(args.day).unwrap_or_else(|| {
        eprintln!("Day {} not implemented yet", args.day);
        std::process::exit(1);
    });

    let input = if args.example {
        utils::input::read_example(args.day)
    } else {
        utils::input::read_input(args.day)
    };

    let run_part = |part_num: u8, part_fn: fn(&dyn Solution, &str) -> String| {
        if args.bench {
            let iterations = 100;
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = part_fn(solution.as_ref(), &input);
            }
            let elapsed = start.elapsed();
            let avg = elapsed / iterations;

            let result = part_fn(solution.as_ref(), &input);
            println!("Part {}: {}", part_num, result);
            println!("Average time ({} runs): {:?}\n", iterations, avg);
        } else {
            let start = Instant::now();
            let result = part_fn(solution.as_ref(), &input);
            let elapsed = start.elapsed();

            println!("Part {}: {}", part_num, result);
            println!("Time: {:?}\n", elapsed);
        }
    };

    match args.part {
        Some(1) => run_part(1, |s, i| s.part1(i)),
        Some(2) => run_part(2, |s, i| s.part2(i)),
        _ => {
            run_part(1, |s, i| s.part1(i));
            run_part(2, |s, i| s.part2(i));
        }
    }
}
