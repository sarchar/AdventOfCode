use std::fs;

#[derive(Copy, Clone)]
enum Rpc { Rock, Paper, Scissors }
#[derive(Copy, Clone)]
enum Outcome { Draw, Win, Lose }

fn play(them: Rpc, you: Rpc) -> u32 {
    match (them, you) {
        (Rpc::Rock, Rpc::Rock)         => 1 + 3,
        (Rpc::Rock, Rpc::Paper)        => 2 + 6,
        (Rpc::Rock, Rpc::Scissors)     => 3 + 0,
        (Rpc::Paper, Rpc::Rock)        => 1 + 0,
        (Rpc::Paper, Rpc::Paper)       => 2 + 3,
        (Rpc::Paper, Rpc::Scissors)    => 3 + 6,
        (Rpc::Scissors, Rpc::Rock)     => 1 + 6,
        (Rpc::Scissors, Rpc::Paper)    => 2 + 0,
        (Rpc::Scissors, Rpc::Scissors) => 3 + 3,
    }
}

fn get_rpc(s: char) -> Rpc {
    match s {
        'A' => Rpc::Rock,
        'B' => Rpc::Paper,
        'C' => Rpc::Scissors,
        _   => Rpc::Rock,
    }
}

fn get_outcome(s: char) -> Outcome {
    match s {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _   => Outcome::Lose,
    }
}

fn get_rpc_for_outcome(them: Rpc, outcome: Outcome) -> Rpc {
    match (them, outcome) {
        (Rpc::Rock, Outcome::Lose) => Rpc::Scissors,
        (Rpc::Paper, Outcome::Lose) => Rpc::Rock,
        (Rpc::Scissors, Outcome::Lose) => Rpc::Paper,
        (Rpc::Rock, Outcome::Draw) => Rpc::Rock,
        (Rpc::Paper, Outcome::Draw) => Rpc::Paper,
        (Rpc::Scissors, Outcome::Draw) => Rpc::Scissors,
        (Rpc::Rock, Outcome::Win) => Rpc::Paper,
        (Rpc::Paper, Outcome::Win) => Rpc::Scissors,
        (Rpc::Scissors, Outcome::Win) => Rpc::Rock,
    }
}

fn main() {
    let mut my_score = 0;

    // parse the input
    for line in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n") {

        if line.len() >= 2 {
            let them    = get_rpc(line.chars().nth(0).unwrap());
            let outcome = get_outcome(line.chars().nth(2).unwrap());
            let you     = get_rpc_for_outcome(them, outcome);
            my_score   += play(them, you);
        }
    }

    println!("total score is {my_score}");
}
