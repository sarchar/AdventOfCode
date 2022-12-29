use std::fs;

//#[derive(Copy, Clone)]
//enum Rpc { Rock, Paper, Scissors }
//#[derive(Copy, Clone)]
//enum Outcome { Draw, Win, Lose }

fn to_pri(ascii: u8) -> usize {
    match ascii {
        0x41..=0x5A => 26 + ascii - 0x41,
        0x61..=0x7A =>      ascii - 0x61,
        _           => 0u8,
    }.into()
}

fn fill_sack(sackstr: &str) -> [u32; 52] {
    let mut sack = [0; 52];
    for &c in sackstr.as_bytes().iter() {
        let pri = to_pri(c);
        sack[pri] = 1;
    }
    sack
}

fn main() {
    let mut pritotal = 0;
    let mut elf_index = 0;
    let mut elf_a = [0; 52];
    let mut elf_b = [0; 52];

    // parse the input
    for line in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        match elf_index { 
            0 => elf_a = fill_sack(line),
            1 => elf_b = fill_sack(line),
            _ => {
                let elf_c = fill_sack(line);
                assert_eq!(elf_a.len(), elf_b.len());
                assert_eq!(elf_b.len(), elf_c.len());

                for i in 0..elf_a.len() {
                    if elf_a[i] != 0 && elf_b[i] != 0 && elf_c[i] != 0 {
                        pritotal += i + 1;
                        break;
                    }
                }

                elf_index = 0;
                continue;
            },
        };

        elf_index += 1;
    }

    println!("{pritotal}");
}
