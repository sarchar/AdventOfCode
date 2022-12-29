use std::fs;

// for day6a set NUM_DIFF to 4, for day6b set to 14
const NUM_DIFF: usize = 14;

fn main() {
    // parse the input
    let file = fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let line = file.split("\r\n").filter(|&s| s.len() > 0).next().expect("Need one line in file");
    let mut backlog = ['?'; NUM_DIFF];

    'letters: for (i, c) in line.chars().enumerate() {
        backlog[i % NUM_DIFF] = c;

        if i < NUM_DIFF - 1 { continue; }

        for check in 0..(NUM_DIFF-1) {
            for against in (check+1)..NUM_DIFF {
                if backlog[check] == backlog[against] {
                    continue 'letters;
                }
            }
        }

        println!("{}", i + 1);
        break;
    }
}

