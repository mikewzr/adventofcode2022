
fn main() {
    let filename = "input.txt";

    let content = std::fs::read_to_string(filename).expect("Cannot read file");
    let content = content.trim_end();
    println!("{}", content);

    let mut calories = vec![0];

    let lines = content.split('\n');
    for line in lines {
        if line.len() > 0 {
            let item = calories.last_mut().unwrap();

            match line.parse::<i32>() {
                Ok(number) => *item += number,
                Err(err) => println!("{}", err),
            }
        }
        else {
            calories.push(0);
        }
    }

    println!();

    for item in &calories {
        println!("{}", item);
    }

    println!();

    println!("Max: {}", calories.iter().max().unwrap());

    calories.sort_unstable_by(|a,b| b.cmp(a));

    println!("Max of the top 3: {}", calories.iter().take(3).sum::<i32>());
}
