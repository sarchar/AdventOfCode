use std::fs;

fn main() {
    let mut elf_index = 1;
    let mut elf_cals = 0;
    let mut most_cals_elf = 0;
    let mut most_cals = 0;

    for x in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n") {

        match x.parse::<u32>() {
            Ok(v) => elf_cals += v,
            Err(_) => {
                if elf_cals > most_cals {
                    most_cals = elf_cals;
                    most_cals_elf = elf_index;
                }
                elf_index += 1;
                elf_cals = 0;
            },
        }
    }

    println!("elf {most_cals_elf} had {most_cals} calories");
}
