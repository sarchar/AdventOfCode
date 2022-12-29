use std::fs;

fn main() {
    let t0 = vec!['F', 'H', 'B', 'V', 'R', 'Q', 'D', 'P'];
    let t1 = vec!['L', 'D', 'Z', 'Q', 'W', 'V'];
    let t2 = vec!['H', 'L', 'Z', 'Q', 'G', 'R', 'P', 'C'];
    let t3 = vec!['R', 'D', 'H', 'F', 'J', 'V', 'B'];
    let t4 = vec!['Z', 'W', 'L', 'C'];
    let t5 = vec!['J', 'R', 'P', 'N', 'T', 'G', 'V', 'M'];
    let t6 = vec!['J', 'R', 'L', 'V', 'M', 'B', 'S'];
    let t7 = vec!['D', 'P', 'J'];
    let t8 = vec!['D', 'C', 'N', 'W', 'V'];
    let mut towers = vec![t0, t1, t2, t3, t4, t5, t6, t7, t8];

    // parse the input
    for line in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        let words: Vec<&str> = line.split(" ").collect();

        let move_size: usize = words[1].parse().unwrap();
        let move_src:  usize = words[3].parse().unwrap();
        let move_dest: usize = words[5].parse().unwrap();
        //println!("Moving {} crates from {} to {}", move_size, move_src, move_dest);

        /*
        // day5a:
        for _ in 0..move_size {
            let src = &mut towers[move_src-1];
            let tmp = src.pop().unwrap();
            //src is no longer used

            let dest = &mut towers[move_dest-1];
            dest.push(tmp);
        }
        */
        // day5b:
        let src = &mut towers[move_src-1];
        let mut tmp = src.split_off(src.len()-move_size);
        //src is no longer used

        let dest = &mut towers[move_dest-1];
        dest.append(&mut tmp);
    }

    for t in towers.iter_mut() {
        print!("{}", t.pop().unwrap());
    }
}

