use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cmp;

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
    let mut scenic_map: Vec<Vec<u32>> = vec![vec![1; map_width]; map_height];

    let mut best_scenic_value = 0u32;
    let mut mul_in = |x, y, s| { 
        println!("{} {} mul_in {}", x, y, s);

        let scenic_map_row: &mut Vec<u32> = scenic_map.get_mut(y).expect("y value is invalid");
        scenic_map_row[x] *= s;

        best_scenic_value = cmp::max(scenic_map_row[x], best_scenic_value);
    };

    // scan each cardinal direction and for each tree continue that direction to find how many
    // trees it can see
    for dir in Direction::iter() {
        // for down and left we need to have an outside loop over incrementing X
        // for left and right we need an outside loop over incrementing Y
        // with inner loops over the other coordinate
        match dir {
            Direction::RIGHT => {
                for y in 0..map_height {
                    for x in 0..map_width-1 {
                        let mut cur_height = map[y][x];
                        let mut num_visible = 0;

                        for x2 in x+1..map_width {
                            let next_val = map[y][x2];

                            if next_val <= cur_height {
                                num_visible += 1;
                            }

                            if next_val >= cur_height { break; }
                        }
                        
                        if num_visible > 1 { mul_in(x, y, num_visible); }
                    }
                }
            },

            Direction::LEFT => { 
                for y in 0..map_height {
                    for x in (1..map_width).rev() {
                        let mut cur_height = map[y][x];
                        let mut num_visible = 0;

                        for x2 in (0..x).rev() {
                            let next_val = map[y][x2];

                            if next_val <= cur_height {
                                num_visible += 1;
                            }

                            if next_val >= cur_height { break; }
                        }
                        
                        if num_visible > 1 { mul_in(x, y, num_visible); }
                    }
                }
            },

            Direction::DOWN => {
                for x in 0..map_width {
                    for y in 0..map_height-1 {
                        let mut cur_height = map[y][x];
                        let mut num_visible = 0;

                        for y2 in y+1..map_height {
                            let next_val = map[y2][x];

                            if next_val <= cur_height {
                                num_visible += 1;
                            }

                            if next_val >= cur_height { break; }
                        }
                        
                        if num_visible > 1 { mul_in(x, y, num_visible); }
                    }
                }
            },

            Direction::UP => { 
                for x in 0..map_width {
                    for y in (1..map_height).rev() {
                        let mut cur_height = map[y][x];
                        let mut num_visible = 0;

                        for y2 in (0..y).rev() {
                            let next_val = map[y2][x];

                            if next_val <= cur_height {
                                num_visible += 1;
                            }

                            if next_val >= cur_height { break; }
                        }
                        
                        if num_visible > 1 { mul_in(x, y, num_visible); }
                    }
                }
            },
        };
    }

    println!("map is {}x{}, best scenic value = {}", map[0].len(), map.len(), best_scenic_value);
}
