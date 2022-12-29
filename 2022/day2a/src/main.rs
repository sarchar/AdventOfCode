use std::fs;

enum Rpc { Rock, Paper, Scissors }

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
        'A' | 'X' => Rpc::Rock,
        'B' | 'Y' => Rpc::Paper,
        'C' | 'Z' => Rpc::Scissors,
        _ => Rpc::Rock,
    }
}

fn main() {
    let mut my_score = 0;

    // parse the input
    for line in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n") {

        if line.len() >= 2 {
            let them = get_rpc(line.chars().nth(0).unwrap());
            let you  = get_rpc(line.chars().nth(2).unwrap());
            my_score += play(them, you);
        }
    }

    println!("total score is {my_score}");
}
