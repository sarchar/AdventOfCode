use std::cmp::Ordering;

#[derive(Debug)]
enum WeirdList<T> {
    Value(T),
    Nested(WeirdListContainer<T>),
}

#[derive(Debug)]
struct WeirdListContainer<T> {
    values: Vec<WeirdList<T>>,
    tag   : u32,
}

// return a WeirdListContainer
fn parse_weird_string_inner(iter: &mut dyn Iterator<Item = char>) -> WeirdListContainer<u32> {
    let mut res: WeirdListContainer<u32> = WeirdListContainer { values: Vec::new(), tag: 0, };

    loop {
        match iter.next() {
            Some('[') => {
                let nested = parse_weird_string_inner(iter);
                let mut should_return = false;
                match iter.next() {
                    Some(',') => { /* keep parsing */ },
                    Some(']') => { /* need to return after this list */ should_return = true; }
                    None      => { /* keep parsing */ },
                    // any other input is invalid
                    Some(c @ _)   => { panic!("invalid input: got {}", c); },
                };
                res.values.push(WeirdList::Nested(nested));
                if should_return { break; }
            },
            Some(']') => {
                // can get an empty array
                break;
            },
            Some(d @ _) => {
                let mut num_str = d.to_string();  
                let mut should_return = true;
                while let Some(x) = iter.next() {
                    if x == ','             { should_return = false; }
                    if x == ',' || x == ']' { break; }
                    num_str.push(x);
                }

                res.values.push(WeirdList::Value(num_str.parse::<u32>().unwrap()));
                if should_return { break; }
            },
            None => {
                // end of string error
                panic!("no matching ] found");
            },
        }
    }

    res
}

// return a WeirdListContainer
fn parse_weird_string(s: &str) -> WeirdListContainer<u32> {
    let inn = s.to_string();
    let mut iter = inn.chars();

    let first = iter.next();
    assert_eq!(first, Some('['));
    parse_weird_string_inner(&mut iter)
}

fn check_order(left: &WeirdListContainer<u32>, right: &WeirdListContainer<u32>) -> Ordering {
    let mut left_iter = left.values.iter();
    let mut right_iter = right.values.iter();

    loop {
        let right_value = right_iter.next();

        match left_iter.next() {
            Some(WeirdList::Value(n)) => {
                match right_value {
                    Some(WeirdList::Value(m)) => {
                        // both values were integers
                        if n < m { return Ordering::Less; }
                        else if n > m { return Ordering::Greater; }
                        // otherwise continue looping through the list
                    },
                    Some(WeirdList::Nested(nested_right)) => {
                        // make left a list and then compare them
                        let new_left_list = WeirdListContainer { values: vec![WeirdList::Value(*n)], tag: 0, };
                        match check_order(&new_left_list, &nested_right) {
                            Ordering::Less    => { return Ordering::Less; },
                            Ordering::Greater => { return Ordering::Greater; },
                            Ordering::Equal   => { },
                        }
                    },
                    None => {
                        // right list ran out first
                        return Ordering::Greater;
                    },
                };
            },

            Some(WeirdList::Nested(nested_left)) => {
                match right_value {
                    Some(WeirdList::Value(m)) => {
                        // make right a list and then compare them
                        let new_right_list = WeirdListContainer { values: vec![WeirdList::Value(*m)], tag: 0, };
                        match check_order(&nested_left, &new_right_list) {
                            Ordering::Less    => { return Ordering::Less; },
                            Ordering::Greater => { return Ordering::Greater; },
                            Ordering::Equal   => { },
                        }
                    },
                    Some(WeirdList::Nested(nested_right)) => {
                        // both values were lists, compare them
                        match check_order(&nested_left, &nested_right) {
                            Ordering::Less    => { return Ordering::Less; },
                            Ordering::Greater => { return Ordering::Greater; },
                            Ordering::Equal   => { },
                        };
                    },
                    None => {
                        // right list ran out first
                        return Ordering::Greater;
                    },
                };
            },

            None => {
                // if there's still something in the right list but we run out in the left, inputs are in order
                match right_value {
                    Some(_) => { return Ordering::Less; },
                    None    => { return Ordering::Equal; },
                };
            },
        };
    }
}

fn main() {
    let mut index_sum = 0;

    let mut weird_list_containers: Vec<WeirdListContainer<u32>> = vec![];

    // parse the file line by line
    for (_y, line) in std::fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt")
        .split("\r\n")
        //.filter(|&s| s.len() > 0)
        .enumerate() {

        match _y % 3 {
            0 => { let left  = parse_weird_string(line); weird_list_containers.push(left);  },
            1 => { let right = parse_weird_string(line); weird_list_containers.push(right); },
            2 => { 
                let len = weird_list_containers.len();
                let left = &weird_list_containers[len - 2];
                let right = &weird_list_containers[len - 1];
                if matches!(check_order(left, right), Ordering::Less) {
                    index_sum += (_y / 3) + 1;
                }
            },
            _ => panic!("invalid"),
        }
    }

    // Part A:
    println!("{index_sum}");

    // Part B:
    // insert [[2]] and [[6]]
    let mut st = parse_weird_string("[[2]]");
    st.tag = 1;
    weird_list_containers.push(st);

    st = parse_weird_string("[[6]]");
    st.tag = 2;
    weird_list_containers.push(st);

    // sort weird_list_containers
    weird_list_containers.sort_by(check_order);
    
    // final loop over to locate the decoders
    let mut start_index = 0;
    let mut end_index = 0;
    for (i, v) in weird_list_containers.iter().enumerate() {
        if v.tag == 1 { start_index = i + 1; }
        if v.tag == 2 { end_index   = i + 1; }
    }

    println!("{}", start_index * end_index);
}
