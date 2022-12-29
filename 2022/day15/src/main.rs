use std::cmp;
use std::fs;
use std::ops::RangeInclusive;
use range_set::RangeSet;

const PART_B: bool = false;
const SAMPLE: bool = false;

struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64,
}

impl Sensor {
    fn new(x: i64, y: i64, beacon_x: i64, beacon_y: i64) -> Sensor {
        Sensor {
            x: x,
            y: y,
            beacon_x: beacon_x,
            beacon_y: beacon_y,
        }
    }

    fn distance_to(&self, x: i64, y: i64) -> u64 {
        return self.x.abs_diff(x) + self.y.abs_diff(y);
    }
    
    fn distance_to_beacon(&self) -> u64 {
        return self.distance_to(self.beacon_x, self.beacon_y);
    }

    fn closest_distance_to_line(&self, y: i64) -> Option<u64> {
        let d = self.y.abs_diff(y);
        if d <= self.distance_to_beacon() {
            Some(d)
        } else {
            None
        }
    }
}

fn main() {
    let mut sensors = Vec::new();

    // read in the sensors and their closest beacons
    for (_y, line) in fs::read_to_string(if SAMPLE { "input2.txt" } else { "input.txt" })
        .expect("could not open input.txt")
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .enumerate() {

        let mut words = line.split_whitespace();
        let x = words.next().unwrap().parse().unwrap();
        let y = words.next().unwrap().parse().unwrap();
        let bx = words.next().unwrap().parse().unwrap();
        let by = words.next().unwrap().parse().unwrap();
        let sensor = Sensor::new(x, y, bx, by);

        sensors.push(sensor);
    }

    // loop over all sensors and determine which elements at y=10 is blocked off

    let (sx, limit) = if PART_B {
        (0, if SAMPLE { 20 } else { 4000000 })
    } else {
        let x = if SAMPLE { 10 } else { 2000000 };
        (x, x)
    };

    for y in sx..=limit {
        let mut res = RangeSet::<[RangeInclusive<i64>; 10]>::new();

        for sensor in &sensors {
            if let Some(d) = sensor.closest_distance_to_line(y) {
                let rem = (sensor.distance_to_beacon() - d) as i64;

                if rem > 0 {
                    // mark values from (sensor.x-rem, y) to (sensor.x+rem, y) as empty
                    if PART_B {
                        let range_start = cmp::min(cmp::max(sensor.x - rem, 0), limit+1);
                        let range_end   = cmp::min(cmp::max(sensor.x + rem, 0), limit+1);
                        //println!("inserting range {}..={}", range_start, range_end);
                        res.insert_range(range_start..=range_end);
                    } else {
                        res.insert_range((sensor.x - rem)..=(sensor.x + rem));
                    }
                } else {
                    // mark (sensor.x, y) as empty
                    if sensor.x >= 0 && sensor.x <= limit {
                        res.insert(sensor.x);
                    }
                }
            }
        }

        // remove all the beacons that are on matching y
        if !PART_B {
            for sensor in &sensors {
                if sensor.beacon_y == y {
                    res.remove(sensor.beacon_x);
                }
            }
        }

        // convert the resulting ranges into a vector and sum up the sizes
        let sv = res.into_smallvec();
        let mut total = 0;

        if !PART_B || sv.len() == 2 {
            for v in &sv {
                total += v.end() - v.start() + 1;
            }

            let tuning_x = sv[0].end() + 1;
            let tuning_frequency = tuning_x * 4000000 + y;

            if !PART_B {
                println!("{total}");
            } else {
                println!("{}", tuning_frequency);
                break;
            }
        }
    }
}
