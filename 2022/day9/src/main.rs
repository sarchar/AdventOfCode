use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn new(knot_count: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); knot_count],
        }
    }

    fn move_head(&mut self, dir: char) -> (i32, i32) {
        match dir {
            'L' => { self.move_left(); },
            'R' => { self.move_right(); },
            'U' => { self.move_up(); },
            'D' => { self.move_down(); },
            _ => { panic!("invalid input"); }
        };
        let knot_len = self.knots.len();
        self.knots[knot_len - 1]
    }

    fn move_right(&mut self) {
        let head = self.knots.get(0).unwrap();
        self.knots[0] = (head.0 + 1, head.1);
        self.catchup();
    }

    fn move_left(&mut self) {
        let head = self.knots.get(0).unwrap();
        self.knots[0] = (head.0 - 1, head.1);
        self.catchup();
    }

    fn move_up(&mut self) {
        let head = self.knots.get(0).unwrap();
        self.knots[0] = (head.0, head.1 - 1);
        self.catchup();
    }

    fn move_down(&mut self) {
        let head = self.knots.get(0).unwrap();
        self.knots[0] = (head.0, head.1 + 1);
        self.catchup();
    }

    fn catchup(&mut self) {
        for i in 1..self.knots.len() { self.catchup_one(i); }
    }

    fn catchup_one(&mut self, which: usize) {
        let dist = self.distance(which);
        if dist < 2 { return (); }
        assert_eq!(dist, 2);

        let head = self.knots[which - 1];
        let tail = self.knots[which];

        // neither X or Y are in alignment, move diagonolly towards the head
        let on_x          = tail.0 == head.0;
        let on_y          = tail.1 == head.1;
        let left_of_head  = tail.0 < head.0;
        let north_of_head = tail.1 < head.1;
        let nx = tail.0 + if left_of_head { 1 } else if on_x { 0 } else { -1 };
        let ny = tail.1 + if north_of_head { 1 } else if on_y { 0 } else { -1 };

        // set the new tail position
        self.knots[which] = (nx, ny);
    }

    // max x or y distance from which to which-1
    fn distance(&self, which: usize) -> i32 {
        assert_eq!(which > 0, true);
        let head = self.knots[which - 1];
        let tail = self.knots[which];
        cmp::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs())
    }
}

fn main() {
    // use '2' for day 9.1, and '10' for day 9.2
    let mut rope = Rope::new(2);

    let mut visited = HashSet::new();
    visited.insert((0, 0)); // starting cell always counts

    // parse the file line by line
    for line in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        let mut words = line.split_whitespace();
        let dir       = words.next().unwrap().chars().nth(0).unwrap();
        let count: u8 = words.next().unwrap().parse().unwrap();

        for _ in 0..count {
            let tail_pos = rope.move_head(dir);
            visited.insert(tail_pos);
        }

        //println!("{} {}, now head:({}, {}) tail:({}, {})", dir, count, rope.hx, rope.hy, rope.tx, rope.ty);
    }

    println!("{}", visited.len());
}
