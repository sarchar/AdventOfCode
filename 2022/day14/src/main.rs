use std::{cmp, fs};

const PART_B: bool = true;

#[derive(Copy, Clone, Debug)]
enum MapSpot {
    Air,
    Rock,
    Sand,
}

struct Map {
    map: Vec<Vec<MapSpot>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        Map {
            map: vec![vec![MapSpot::Air; width]; height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&MapSpot> {
        let map_row = self.map.get(y)?;
        return Some(map_row.get(x)?);
    }

    fn set(&mut self, x: usize, y: usize, ms: MapSpot) -> Result<MapSpot, String> {
        let map_row = self.map.get_mut(y).ok_or("Y out of bounds").unwrap();
        let map_elem = map_row.get_mut(x).ok_or("X out of bounds").unwrap();
        *map_elem = ms;
        Ok(ms)
    }
}

fn main() {
    let mut line_strips = Vec::new();
    let mut min_x = u32::MAX; 
    let mut min_y = u32::MAX;
    let mut map_width  = u32::MIN;
    let mut map_height = u32::MIN;

    // read in the line strips
    for (_y, line) in fs::read_to_string("input.txt")
        .expect("could not open input.txt")
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .enumerate() {

        let mut line_strip = Vec::new();
        for (_x, segment) in line.split("->").enumerate() {
            let mut elems = segment.trim().split(",");
            let x = elems.next().unwrap().parse::<u32>().unwrap();
            let y = elems.next().unwrap().parse::<u32>().unwrap();

            line_strip.push((x, y));

            min_x      = cmp::min(min_x, x);
            min_y      = cmp::min(min_y, y);
            map_width  = cmp::max(map_width, x);
            map_height = cmp::max(map_height, y);
        }

        line_strips.push(line_strip);
    }

    // fixup the map size
    map_width  = map_width + 1;
    map_height = map_height + 1;

    // add the floor to the map_height value and extend the map vary far
    if PART_B {
        map_width *= 3;
        map_height += 2;
    }

    // create an empty map
    println!("map is {map_width}x{map_height}");
    //let mut map = vec![vec![MapSpot::Air; map_width as usize]; map_height as usize];
    let mut map = Map::new(map_width as usize, map_height as usize);

    // trace the paths to fill out the rock in the map
    for line_strip in &line_strips {
        let mut segments = line_strip.iter();
        let mut cur = segments.next().unwrap();
        for next in segments {
            //println!("{:?}->{:?}", cur, next);

            if cur.0 != next.0 { // horizontal line
                assert_eq!(cur.1, next.1);

                let sx = cmp::min(cur.0, next.0);
                let ex = cmp::max(cur.0, next.0);
                for x in sx..=ex {
                    map.set(x as usize, cur.1 as usize, MapSpot::Rock).expect("must be OK");
                    //let map_row = &mut map[cur.1 as usize];
                    //map_row[x as usize] = MapSpot::Rock;
                }
            } else { // vertical line
                assert_eq!(cur.1 != next.1, true);

                let sy = cmp::min(cur.1, next.1);
                let ey = cmp::max(cur.1, next.1);
                for y in sy..=ey {
                    map.set(cur.0 as usize, y as usize, MapSpot::Rock).expect("must be OK");
                    //let map_row = &mut map[y as usize];
                    //map_row[cur.0 as usize] = MapSpot::Rock;
                }
            }

            cur = next;
        }
    }

    // set the floor to rock for part b
    if PART_B {
        for x in 0..map_width {
            map.set(x as usize, (map_height - 1) as usize, MapSpot::Rock).expect("must be OK");
        }
    }

    // simulate sand falling from 500,0 until one falls off the map
    // or until the sand source is blocked (not a problem for part a)
    let mut num_sand = 0;

    'outer: loop {
        let mut sand_x = 500;
        let mut sand_y = 0;

        // break once sand blocks the source
        if matches!(map.get(sand_x, sand_y).unwrap(), MapSpot::Sand) { break; }

        loop {
            // look at the spot below the sand to determine how it will fall
            match map.get(sand_x, sand_y + 1) {
                Some(MapSpot::Air) => {
                    // below is air, so we can move down easily
                    sand_y += 1;
                },

                None => {
                    break 'outer;
                },

                // if Rock or Sand is below, the new sand attemps to move diagonally left
                Some(MapSpot::Rock)|Some(MapSpot::Sand) => {
                    match map.get(sand_x - 1, sand_y + 1) {
                        // if that spot is air, we can successfully move
                        Some(MapSpot::Air) => {
                            sand_x -= 1;
                            sand_y += 1;
                        },

                        None => {
                            break 'outer;
                        },

                        // if that tile is blocked, we attempt to move down and to the right
                        Some(MapSpot::Rock)|Some(MapSpot::Sand) => {
                            match map.get(sand_x + 1, sand_y + 1) {
                                // if that spot is air, we can successfully move
                                Some(MapSpot::Air) => {
                                    sand_x += 1;
                                    sand_y += 1;
                                },

                                None => {
                                    break 'outer;
                                },

                                // if Rock or Sand is present, the new sand comes to rest and stops here
                                Some(MapSpot::Rock)|Some(MapSpot::Sand) => {
                                    map.set(sand_x, sand_y, MapSpot::Sand).expect("must be OK");
                                    num_sand += 1;
                                    break;
                                },
                            };
                        },
                    };
                },
            }
        }

        //// lets print (a section of) the map!
        //for y in 0..map_height {
        //    for x in 470..525 {
        //        match map.get(x as usize, y as usize) {
        //            Some(MapSpot::Air) => { print!("."); },
        //            Some(MapSpot::Rock) => { print!("#"); },
        //            Some(MapSpot::Sand) => { print!("o"); },
        //            None => { },
        //        }
        //    }
        //    println!("");
        //}
        //println!("");
    }

    println!("{num_sand}");
}
