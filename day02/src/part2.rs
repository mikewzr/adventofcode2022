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

    fn looses_against(&self) -> RPS {
        match *self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn wins_against(&self) -> RPS {
        match *self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum Strategy {
    Win,
    Draw,
    Loose,
}

impl Strategy {
    fn value(&self) -> i32 {
        match *self {
            Strategy::Win => 6,
            Strategy::Draw => 3,
            Strategy::Loose => 0,

        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

fn calc_points(theirs: &RPS, strat: &Strategy) -> i32 {
    match strat {
        Strategy::Win => theirs.looses_against().value() + strat.value(),
        Strategy::Draw => theirs.value() + strat.value(),
        Strategy::Loose => theirs.wins_against().value() + strat.value(),
    }
}

pub fn part2(filename: &String) {
    let content = std::fs::read_to_string(filename).expect("Cannot read file");
    let content = content.trim_end();

    let input = content.split('\n').map(
        |l| {
            let mut it = l.split(' ');
            let theirs = match it.next().unwrap() {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("Unknown RPS value"),
            };
            let strategy = match it.next().unwrap() {
                "X" => Strategy::Loose,
                "Y" => Strategy::Draw,
                "Z" => Strategy::Win,
                _ => panic!("Unknown strategy"),
            };

            (theirs, strategy)
        }

    ).collect::<Vec<_>>();

    let mut points = 0;

    for t in input {
        //println!("{t:?}");
        points += calc_points(&t.0, &t.1);
    }

    println!("Total points: {points}");
}
