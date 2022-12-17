mod part1;

use std::process::exit;
use std::env;

fn usage(program_name: &String) {
    println!("Usage: {program_name} <file>");
    exit(0);
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage(args.first().unwrap());
    }

    part1::part1(&args[1]);
}
