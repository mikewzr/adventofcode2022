mod part1;
mod part2;

use std::{env, process::exit};

fn usage(program_name: &String) {
    println!("Usage: {program_name} <part> <file>");
    exit(0);
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        usage(args.first().unwrap());
    }

    match args[1].parse::<i32>() {
        Ok(1) => part1::part1(&args[2]),
        Ok(2) => part2::part2(&args[2]),
        _ => usage(args.first().unwrap()),
    };
}
