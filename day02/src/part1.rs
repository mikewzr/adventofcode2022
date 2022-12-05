use std::fmt;

// RockPaperScissors
#[derive(Debug)]
#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn value(&self) -> i32 {
        match *self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

fn calc_points(tuple: &(RPS, RPS)) -> i32 {
    let ours = &tuple.1;

    if tuple.0 == tuple.1 {
        return ours.value() + 3;
    }
    else {
        return match tuple {
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => ours.value() + 6,
            _ => ours.value(),
        };
    }
}

pub fn part1() {
    let filename = "input.txt";
    let content = std::fs::read_to_string(filename).expect("Cannot read file");
    let content = content.trim_end();

    let input = content.split('\n').map(
        |l| {
            let mut it = l.split(' ').map(
                |c|
                match c {
                "A"|"X" => RPS::Rock,
                "B"|"Y" => RPS::Paper,
                "C"|"Z" => RPS::Scissors,
                _ => panic!("Unknown value"),
                }
            );

            (it.next().unwrap(), it.next().unwrap())
        }

    ).collect::<Vec<_>>();

    let mut points = 0;

    for t in input {
        //println!("{t:?}");
        points += calc_points(&t);
    }

    println!("Total points: {points}");
}
