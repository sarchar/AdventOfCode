use std::fs;

struct Range {
    pub start: u32,
    pub end:   u32,
}

fn parse_range(s: &str) -> Range {
    let (ln, rn) = s.split_once("-").unwrap();
    Range { start: ln.parse::<u32>().unwrap(), end: rn.parse::<u32>().unwrap() }
}

fn main() {
    let mut count = 0;

    // parse the input
    for line in fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        let (left, right) = line.split_once(",").unwrap();
        let lrange = parse_range(left);
        let rrange = parse_range(right);

        // day1a:
        /*
        if (rrange.start >= lrange.start && rrange.end <= lrange.end)
         || (lrange.start >= rrange.start && lrange.end <= rrange.end) {
             count += 1;
        }
        */
        // day1b:
        if (rrange.start >= lrange.start && rrange.start <= lrange.end)
         || (lrange.start >= rrange.start && lrange.start <= rrange.end) {
             count += 1;
        }
    }

    println!("{count}");
}

