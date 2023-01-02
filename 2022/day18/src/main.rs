use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

const SAMPLE: bool = false;

struct Cube {
    u: bool,
    d: bool,
    l: bool,
    r: bool,
    f: bool,
    b: bool,
}

impl Cube {
    fn new() -> Cube {
        Cube {
            u: true, d: true,
            l: true, r: true,
            f: true, b: true,
        }
    }
}

struct Space {
    cubes        : HashMap<(i32, i32, i32), Cube>,
    visible_faces: u32,
    bbox         : ((i32, i32, i32), (i32, i32, i32)),
}

impl Space {
    fn new() -> Space {
        Space {
            cubes: HashMap::new(),
            visible_faces: 0,
            bbox         :
                ((i32::MAX, i32::MAX, i32::MAX),
                 (i32::MIN, i32::MIN, i32::MIN)),
        }
    }

    fn add_cube(&mut self, x: i32, y: i32, z: i32) {
        let mut new_cube = Cube::new();
        let mut new_faces = 6;

        let ((mut x_min, mut y_min, mut z_min),
             (mut x_max, mut y_max, mut z_max)) = self.bbox;

        x_min = cmp::min(x_min, x);
        y_min = cmp::min(y_min, y);
        z_min = cmp::min(z_min, z);
        x_max = cmp::max(x_max, x);
        y_max = cmp::max(y_max, y);
        z_max = cmp::max(z_max, z);

        self.bbox = ((x_min, y_min, z_min), (x_max, y_max, z_max));

        // check up
        match self.cubes.get_mut(&(x, y + 1, z)) {
            Some(other) => {
                if other.d {
                    other.d = false;
                    self.visible_faces -= 1;
                }
                new_cube.u = false;
                new_faces -= 1;
            },
            _ => {},
        }

        // check down
        match self.cubes.get_mut(&(x, y - 1, z)) {
            Some(other) => {
                if other.u {
                    other.u = false;
                    self.visible_faces -= 1;
                }
                new_cube.d = false;
                new_faces -= 1;
            },
            _ => {},
        }

        // check left
        match self.cubes.get_mut(&(x - 1, y, z)) {
            Some(other) => {
                if other.r {
                    other.r = false;
                    self.visible_faces -= 1;
                }
                new_cube.l = false;
                new_faces -= 1;
            },
            _ => {},
        }

        // check right
        match self.cubes.get_mut(&(x + 1, y, z)) {
            Some(other) => {
                if other.l {
                    other.l = false;
                    self.visible_faces -= 1;
                }
                new_cube.r = false;
                new_faces -= 1;
            },
            _ => {},
        }

        // check front
        match self.cubes.get_mut(&(x, y, z + 1)) {
            Some(other) => {
                if other.b {
                    other.b = false;
                    self.visible_faces -= 1;
                }
                new_cube.f = false;
                new_faces -= 1;
            },
            _ => {},
        }

        // check back
        match self.cubes.get_mut(&(x, y, z - 1)) {
            Some(other) => {
                if other.f {
                    other.f = false;
                    self.visible_faces -= 1;
                }
                new_cube.b = false;
                new_faces -= 1;
            },
            _ => {},
        }

        // add the cube to the set
        self.cubes.insert((x, y, z), new_cube);
        self.visible_faces += new_faces;
    }
}

fn main() {
    let mut space = Space::new();

    // read in the sensors and their closest beacons
    for (_line_index, line) in fs::read_to_string(if SAMPLE { "input2.txt" } else { "input.txt" })
        .expect("could not open input")
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .enumerate() {

        let mut words = line.split(",");
        let x = words.next().unwrap().parse::<i32>().unwrap();
        let y = words.next().unwrap().parse::<i32>().unwrap();
        let z = words.next().unwrap().parse::<i32>().unwrap();

        space.add_cube(x, y, z);
    }

    println!("faces: {}", space.visible_faces);

    let crawl_paths: HashMap<char, [[_; 3]; 4]> = HashMap::from([
        // for the front face
        ('f', [ // go right
                [|x: i32, y: i32, z: i32| (x + 1, y, z + 1, 'l'),
                 |x: i32, y: i32, z: i32| (x + 1, y, z, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'r')
                ],
                // go up
                [|x: i32, y: i32, z: i32| (x, y + 1, z + 1, 'd'),
                 |x: i32, y: i32, z: i32| (x, y + 1, z, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'u') ],
                // go left
                [|x: i32, y: i32, z: i32| (x - 1, y, z + 1, 'r'),
                 |x: i32, y: i32, z: i32| (x - 1, y, z, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'l') ],
                // go down
                [|x: i32, y: i32, z: i32| (x, y - 1, z + 1, 'u'),
                 |x: i32, y: i32, z: i32| (x, y - 1, z, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'd')
                ]
              ]),
        // for the right face
        ('r', [ // go right
                [|x: i32, y: i32, z: i32| (x + 1, y, z - 1, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z - 1, 'r'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'b')
                ],
                // go up
                [|x: i32, y: i32, z: i32| (x + 1, y + 1, z, 'd'),
                 |x: i32, y: i32, z: i32| (x, y + 1, z, 'r'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'u') ],
                // go left
                [|x: i32, y: i32, z: i32| (x + 1, y, z + 1, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z + 1, 'r'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'f') ],
                // go down
                [|x: i32, y: i32, z: i32| (x + 1, y - 1, z, 'u'),
                 |x: i32, y: i32, z: i32| (x, y - 1, z, 'r'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'd')
                ]
              ]),
        // for the upper/top face
        ('u', [ // go right
                [|x: i32, y: i32, z: i32| (x + 1, y + 1, z, 'l'),
                 |x: i32, y: i32, z: i32| (x + 1, y, z, 'u'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'r')
                ],
                // go up
                [|x: i32, y: i32, z: i32| (x, y + 1, z - 1, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z - 1, 'u'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'b') ],
                // go left
                [|x: i32, y: i32, z: i32| (x - 1, y + 1, z, 'r'),
                 |x: i32, y: i32, z: i32| (x - 1, y, z, 'u'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'l') ],
                // go down
                [|x: i32, y: i32, z: i32| (x, y + 1, z + 1, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z + 1, 'u'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'f')
                ]
              ]),
        // for the left face
        ('l', [ // go right
                [|x: i32, y: i32, z: i32| (x - 1, y, z + 1, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z + 1, 'l'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'f')
                ],
                // go up
                [|x: i32, y: i32, z: i32| (x - 1, y + 1, z, 'd'),
                 |x: i32, y: i32, z: i32| (x, y + 1, z, 'l'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'u') ],
                // go left
                [|x: i32, y: i32, z: i32| (x - 1, y, z - 1, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z - 1, 'l'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'b') ],
                // go down
                [|x: i32, y: i32, z: i32| (x - 1, y - 1, z, 'u'),
                 |x: i32, y: i32, z: i32| (x, y - 1, z, 'l'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'd')
                ]
              ]),
        // for the bottom face
        ('d', [ // go right
                [|x: i32, y: i32, z: i32| (x + 1, y - 1, z, 'l'),
                 |x: i32, y: i32, z: i32| (x + 1, y, z, 'd'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'r')
                ],
                // go up
                [|x: i32, y: i32, z: i32| (x, y - 1, z + 1, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z + 1, 'd'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'f') ],
                // go left
                [|x: i32, y: i32, z: i32| (x - 1, y - 1, z, 'r'),
                 |x: i32, y: i32, z: i32| (x - 1, y, z, 'd'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'l') ],
                // go down
                [|x: i32, y: i32, z: i32| (x, y - 1, z - 1, 'f'),
                 |x: i32, y: i32, z: i32| (x, y, z - 1, 'd'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'b')
                ]
              ]),
        // for the back face
        ('b', [ // go right
                [|x: i32, y: i32, z: i32| (x - 1, y, z - 1, 'r'),
                 |x: i32, y: i32, z: i32| (x - 1, y, z, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'l')
                ],
                // go up
                [|x: i32, y: i32, z: i32| (x, y + 1, z - 1, 'd'),
                 |x: i32, y: i32, z: i32| (x, y + 1, z, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'u') ],
                // go left
                [|x: i32, y: i32, z: i32| (x + 1, y, z - 1, 'l'),
                 |x: i32, y: i32, z: i32| (x + 1, y, z, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'r') ],
                // go down
                [|x: i32, y: i32, z: i32| (x, y - 1, z - 1, 'u'),
                 |x: i32, y: i32, z: i32| (x, y - 1, z, 'b'),
                 |x: i32, y: i32, z: i32| (x, y, z, 'd')
                ]
              ])
    ]);


    // find a single face to start from
    let wanted_z = space.bbox.1.2;
    for (&(x, y, z), _) in space.cubes.iter() {
        if z != wanted_z { continue; }

        //println!("start searching from {},{},{},f", x, y, z);

        let mut to_visit = VecDeque::new();
        to_visit.push_back((x, y, z, 'f'));

        let mut visited = HashSet::new();

        while to_visit.len() > 0 {
            let where_we_are = to_visit.pop_front().unwrap();

            // some places will get added twice, double check here
            if visited.contains(&where_we_are) { continue; }

            visited.insert(where_we_are.clone());

            if let Some(arr) = crawl_paths.get(&where_we_are.3) { // get the crawl method for the face we're on
                // arr is a list of length 4, one for each cardinal direction
                for crawldir in arr {
                    // crawldir is lengh 3, for the 3 possible edges we could traverse to
                    // (in order of priority)
                    for dirfunc in crawldir {
                        let (x, y, z, face) = dirfunc(where_we_are.0, where_we_are.1, where_we_are.2);
                        if space.cubes.contains_key(&(x, y, z)) {
                            if !visited.contains(&(x, y, z, face)) {
                                to_visit.push_back((x, y, z, face));
                            }

                            // the presence of the block in this direction prevents us from checking the others
                            break;
                        }
                    }
                }
            } else {
                todo!();
            }
        }

        println!("faces exposed: {}", visited.len());
        break;
    }

}
