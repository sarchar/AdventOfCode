use std::cmp;
use std::collections::VecDeque;

const PART_B: bool = true;

fn best_distance(map: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> Option<u32> {

    // need an empty visited map
    let map_height = map.len();
    let map_width  = map.get(0).unwrap().len();
    let mut distance_map = vec![vec![None; map_width]; map_height];

    // reset start position to distance of 0
    {
        let distance_row = distance_map.get_mut(start.1).unwrap();
        distance_row[start.0] = Some(0);
    }

    let mut to_check = VecDeque::new();
    to_check.push_back(start);

    while let Some(current_position) = to_check.pop_front() {
        if current_position == goal { continue; }

        let current_elevation = {
            let map_row = map.get(current_position.1).unwrap();
            *map_row.get(current_position.0).unwrap()
        };

        let current_distance = {
            let distance_row = distance_map.get(current_position.1).unwrap();
            (*distance_row.get(current_position.0).unwrap()).unwrap()
        };

        let mut check_dir = |nx: usize, ny: usize| {
            if let Some(map_row) = map.get(ny) { // try to get the row
                let distance_row = distance_map.get_mut(ny).unwrap(); // if the row exists, the visited map must be valid
 
                // try to index the x location
                if let Some(&x) = map_row.get(nx) {
                    let dest_distance = if let Some(x) = *distance_row.get(nx).unwrap() { x } else { u32::MAX };
 
                    // if not visited and elevation is valid, visit it and set the minimum value
                    if (current_distance + 1) < dest_distance && (x == current_elevation + 1 || x <= current_elevation) {
                        distance_row[nx] = Some(current_distance + 1);
                        to_check.push_back((nx, ny));
                    }
                }
            }
        };

        check_dir(current_position.0 + 1, current_position.1    );
        check_dir(current_position.0    , current_position.1 + 1);
        if current_position.0 > 0 { check_dir(current_position.0 - 1, current_position.1); };
        if current_position.1 > 0 { check_dir(current_position.0    , current_position.1 - 1); };
    };

    let distance_row = distance_map.get(goal.1).unwrap();
    distance_row[goal.0]
}

fn main() {
    let mut map = Vec::new();
    let mut goal = (0, 0);
    let mut starting_points = VecDeque::new();

    // parse the file line by line
    for (_y, line) in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        .filter(|&s| s.len() > 0)
        .enumerate() {

        let mut row = Vec::new();

        for (_x, c) in line.chars().enumerate() {
            let b = match c {
                'S' => {
                    if !PART_B {
                        starting_points.push_back((_x, _y));
                    }
                    0
                },
                'E' => {
                    goal = (_x, _y);
                    25
                },
                _ => {
                    let d = (c as i32) - 0x61;
                    if d == 0 && PART_B {
                        starting_points.push_back((_x, _y));
                    }
                    d
                },
            };

            row.push(b as u8);
        }

        map.push(row);
    }

    let mut min_end_distance = u32::MAX;
    while let Some(start) = starting_points.pop_front() {
        min_end_distance = cmp::min(min_end_distance, match best_distance(&map, start, goal) {
            Some(end_distance) => { end_distance },
            None               => { u32::MAX },
        });
    }

    println!("min distance to goal is {}", min_end_distance);
}
