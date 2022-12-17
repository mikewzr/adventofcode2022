use core::panic;
use itertools::Itertools;

fn get_priority(c: char) -> Result<i32, &'static str>
{
    match c {
        'a'..='z' => Ok(c as i32 - 'a' as i32 + 1),
        'A'..='Z' => Ok(c as i32 - 'A' as i32 + 27),
        _ => Err("Char is not in a valid range"),
    }
}

pub fn part1(filename: &String)
{
    let content = std::fs::read_to_string(filename).expect("Could not read file");
    let content = content.trim_end();

    let lines = content.split('\n');
    let mut rucksacks = Vec::<(String, String)>::new();
    for line in lines {
        if line.len() % 2 != 0 {
            panic!("Line length is odd");
        }

        let half_len = line.len() / 2;
        rucksacks.push((line[0..half_len].to_string(), line[half_len..line.len()].to_string()));
    }

    let mut priority_sum = 0;

    for rucksack in rucksacks {
        let unique = rucksack.0.chars().unique();
        let mut items_in_both_compartments = Vec::new();

        for c in unique {
            if rucksack.1.contains(c) {
                items_in_both_compartments.push(c);
                if let Ok(priority) = get_priority(c) {
                    priority_sum += priority;
                }
            }
        }
    }

    println!("Sum: {priority_sum}");
}
