use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug,EnumIter)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

fn main() {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    // read in the file and create a line parser
    for line in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0) {

        map.push(Vec::new());

        for c in line.chars() {
            let map_len = map.len();
            map[map_len-1].push(c.to_string().parse::<u8>().expect("must be a number"));
        }
    }

    // need a map of visibilities
    let map_height = map.len();
    let map_width  = map[0].len();

    // create a map filled with 0s
    let mut visible_map: Vec<Vec<u8>> = vec![vec![0; map_width]; map_height];

    let mut visible_count = 0u32;
    let mut mark_visible = |x, y| { 
        println!("{} {} is visible", x, y);

        let visible_map_row: &mut Vec<u8> = visible_map.get_mut(y).expect("y value is invalid");
        if visible_map_row[x] == 0 { visible_count += 1; }
        visible_map_row[x] = 1;
    };

    // scan each cardinal direction over the map to find where visibility stops
    for dir in Direction::iter() {
        // for down and left we need to have an outside loop over incrementing X
        // for left and right we need an outside loop over incrementing Y
        // with inner loops over the other coordinate
        match dir {
            Direction::RIGHT => {
                for y in 1..=map_height-2 {
                    let mut cur_height = map[y][0];

                    for x in 1..=map_width-2 {
                        let next_val = map[y][x];
                        if next_val > cur_height {
                            mark_visible(x, y);
                            cur_height = next_val;
                        }

                        if cur_height == 9 { break; }
                    }
                }
            },

            Direction::LEFT => { 
                for y in 1..=map_height-2 {
                    let mut cur_height = map[y][map_width-1];

                    for x in (1..=map_width-2).rev() {
                        let next_val = map[y][x];
                        if next_val > cur_height {
                            mark_visible(x, y);
                            cur_height = next_val;
                        }

                        if cur_height == 9 { break; }
                    }
                }
            },

            Direction::DOWN => {
                for x in 1..=map_width-2 {
                    let mut cur_height = map[0][x];

                    for y in 1..=map_height-2 {
                        let next_val = map[y][x];
                        if next_val > cur_height {
                            mark_visible(x, y);
                            cur_height = next_val;
                        }

                        if cur_height == 9 { break; }
                    }
                }
            },

            Direction::UP => { 
                for x in 1..=map_width-2 {
                    let mut cur_height = map[map_height-1][x];
                    for y in (1..=map_height-2).rev() {

                        let next_val = map[y][x];
                        if next_val > cur_height {
                            mark_visible(x, y);
                            cur_height = next_val;
                        }

                        if cur_height == 9 { break; }
                    }
                }
            },
        };
    }

    let bonus = 2 * (map_width - 1 + map_height - 1) as u32;
    println!("map is {}x{}, visible count = {}", map[0].len(), map.len(), visible_count + bonus);
}
