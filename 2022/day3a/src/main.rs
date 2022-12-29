use std::fs;

//#[derive(Copy, Clone)]
//enum Rpc { Rock, Paper, Scissors }
//#[derive(Copy, Clone)]
//enum Outcome { Draw, Win, Lose }

fn to_pri(ascii: u8) -> usize {
    match ascii {
        0x41..=0x5A => (26 + ascii - 0x41).into(),
        0x61..=0x7A => (     ascii - 0x61).into(),
        _ => 0u8.into(),
    }
}

fn main() {
    let mut pritotal = 0;

    // parse the input
    for line in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        let (left, right) = line.split_at(line.len() / 2);
        assert_eq!(left.len(), right.len());

        let mut lsack = [0; 52];
        let mut rsack = [0; 52];

        for (&lb, &rb) in left.as_bytes().iter().zip(right.as_bytes().iter()) {
            let lpri = to_pri(lb);
            let rpri = to_pri(rb);

            // set the compartments as having the item
            lsack[lpri] = 1;
            rsack[rpri] = 1;

            // check if the other compartment has this item
            if lsack[rpri] == 1 {
                pritotal += rpri + 1;
                break;
            } else if rsack[lpri] == 1 {
                pritotal += lpri + 1;
                break;
            }
        }
    }

    println!("{pritotal}");
}
