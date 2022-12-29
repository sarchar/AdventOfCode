use std::cmp;
use std::collections::HashMap;
use std::fs;

const SAMPLE : bool = false;
const VERBOSE: bool = false;

#[derive(Clone,Debug)]
struct Room {
    valve: u32,
    connections: Vec<usize>,
    name: String,
}

#[derive(Clone,Debug)]
struct Map {
    rooms: Vec<Room>,
}


impl Map {
    fn new() -> Map {
        Map {
            rooms: Vec::new(),
        }
    }

    fn new_room(&mut self, name: String) -> usize {
        let new_room = Room { valve: 0, connections: Vec::new(), name: name };
        self.rooms.push(new_room);
        self.rooms.len() - 1
    }

    fn set_valve(&mut self, room_id: usize, valve: u32) {
        self.rooms[room_id].valve = valve;
    }

    fn add_connection(&mut self, room_id: usize, connected_room_id: usize) {
        self.rooms[room_id].connections.push(connected_room_id);
    }
}

// return the (best) total amount of flow that we can achieve after time_left time
// try traversing all the connected rooms twice. first check if we get better results by skipping
// the valve in this room (if it's non-zero), and then check if it's worth using one time unit on
// the valve to increase flow before leaving this room
fn best_flow(map: &mut Map, current_room_id: usize, from_room_id: usize, current_flow_rate: u32, remaining_valves: u32, time_left: u8) -> u32 {
    // if there's no more time left, then we're done
    if time_left == 0 { return 0; }

    // if there's no more valve pressure left at all, we can shortcut the rest of our search
    if remaining_valves == 0 {
        return (time_left as u32) * current_flow_rate;
    }

    let room_connections = {
        let room: &Room = map.rooms.get(current_room_id).unwrap();

        if VERBOSE {
            for _ in 0..(30-time_left) { print!("."); }
            println!("in room {} (valve={}) at time_left={}, current_flow_rate={}", room.name, room.valve, time_left, current_flow_rate);
        }

        room.connections.clone()
    };

    let mut best = u32::MIN;
    for &connection_room_id in &room_connections {
        // don't return into the room we just came from (without activating the valve here first)
        if connection_room_id == from_room_id { continue; }

        // does moving into room connection_room_id (costing 1 time unit and producing current_flow_rate)
        // produce a better outcome than we already have going to other rooms?
        best = cmp::max(
            best,
            best_flow(map, connection_room_id, current_room_id, current_flow_rate, remaining_valves, time_left - 1)
                + current_flow_rate
        );
    }

    // if there's no flow in this room we don't need to check further
    {
        let room: &Room = map.rooms.get(current_room_id).unwrap();
        if room.valve == 0 || time_left == 1 { return best; }
    }

    // now turn on the valve in this room (reducing the valve value to zero and increasing flow)
    let (current_room_valve, new_flow_rate) = {
        let room = map.rooms.get_mut(current_room_id).unwrap();

        let current_room_valve = room.valve;
        room.valve = 0;

        if VERBOSE {
            for _ in 0..(30-time_left) {
                print!(".");
            }
            println!("opening {} for value {} at time {} (remaining_valves={})", room.name, current_room_valve, time_left, remaining_valves);
        }

        (current_room_valve, current_flow_rate + current_room_valve)
    };

    // recheck all the connections again (this time allowing us to return to where we came since
    // we opened the valve in this room) using two time units (one to move, one to open the valve)
    //let mut best_with_opening = u32::MIN;
    for &connection_room_id in &room_connections {
        // does moving into room connection_room_id (costing 1 time unit and producing new_flow_rate)
        // produce a better outcome than we already have going to other rooms?
        best = cmp::max(
            best,
            best_flow(map, connection_room_id, current_room_id, new_flow_rate, 
                      remaining_valves - current_room_valve, time_left - 2)
                        + current_flow_rate + new_flow_rate   // one minute opening the valve releases the old value
                                                              // and the next, traversing to the room releases the new amount
        );
    }

    // restore the room to its original state so we can keep checking the rest of the paths
    {
        let room = map.rooms.get_mut(current_room_id).unwrap();
        room.valve = current_room_valve;
        assert_ne!(room.valve, 0);

        if VERBOSE {
            for _ in 0..(30-time_left) {
                print!(".");
            }
            println!("undoing valve {} (value {})", room.name, room.valve);
        }
    }

    best
}

fn main() {
    let mut room_map: HashMap<&str, usize> = HashMap::new();
    let mut map = Map::new();
    let mut start_room_id = 0;
    let mut total_valves = 0;

    // read in the sensors and their closest beacons
    for (_y, line) in fs::read_to_string(if SAMPLE { "input2.txt" } else { "input.txt" })
        .expect("could not open input.txt")
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .enumerate() {

        let mut words = line.split_whitespace();
        let room_name = words.next().unwrap();
        let valve = words.next().unwrap().parse().unwrap();

        // create or get the room_id
        let room_id = if room_map.contains_key(&room_name) {
            room_map.get(&room_name).cloned().unwrap()
        } else {
            let x = map.new_room(room_name.to_string());
            room_map.insert(room_name.clone(), x);
            x
        };

        // update the valve value on this room
        map.set_valve(room_id, valve);
        total_valves += valve;

        // make all the room connections
        for connected_room_name in words {
            let connected_room_id = if room_map.contains_key(&connected_room_name) {
                room_map.get(&connected_room_name).cloned().unwrap()
            } else {
                let x = map.new_room(connected_room_name.to_string());
                room_map.insert(connected_room_name.clone(), x);
                x
            };

            map.add_connection(room_id, connected_room_id);
        }

        if room_name == "AA" {
            start_room_id = room_id;
        }
    }

    let best = best_flow(&mut map, start_room_id, usize::MAX, 0, total_valves, 30);
    println!("{best}");
}
