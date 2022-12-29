use std::fs;

fn main() {
    let mut elf_cals = 0;
    let mut elves = Vec::new();

    // parse the input
    for x in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n") {

        match x.parse::<u32>() {
            Ok(v) => elf_cals += v,
            Err(_) => {
                elves.push(elf_cals);
                elf_cals = 0
            },
        }
    }

    // sort the elves
    elves.sort();
    elves.reverse();

    // sum the first 3 elements
    let result = elves.iter().take(3).fold(0, |acc, &x| acc + x);

    println!("total calories for top 3 elves: {result}")
}
