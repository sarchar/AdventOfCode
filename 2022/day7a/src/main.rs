use std::collections::HashMap;

enum DirMovement {
    Up,
    Root,
    EOF
}

struct File {
    name:        String,
    size:        u32,
}

struct Directory {
    name:        String,
    directories: HashMap<String, Directory>,
    files:       HashMap<String, File>,
    size:        u32,
}

fn parse_commands<'a, I>(full_path: String, current_directory: &mut Directory, lines: &mut I) 
    -> Result<DirMovement, &'static str> 
where
    I: Iterator<Item = &'a str> {

    loop {
        // split the line into words
        let line = lines.next();
        if line.is_none() {
            return Ok(DirMovement::EOF);
        }

        let mut words = line.unwrap().split_whitespace();

        // look at the first word
        match words.next() {
            // handle '$' commands by looking at the next word
            Some("$") => match words.next() {
                // handle 'cd /', 'cd ..', and 'cd <dir>'
                Some("cd") => match words.next() {
                    Some("..") => { 
                        println!("cd ..");
                        return Ok(DirMovement::Up);
                    },
                    Some("/")  => { 
                        println!("cd /");
                        return Ok(DirMovement::Root);
                    },

                    to_dir @ _ => { 
                        let to_dir_str = to_dir.unwrap().to_string();
                        let sub_directory = current_directory.directories.get_mut(&to_dir_str).unwrap();
                        println!("cd {}", to_dir_str);

                        let new_path = format!("{}/{}", full_path, to_dir_str);
                        match parse_commands(new_path, sub_directory, lines) {
                            // for Root commands, continue returning
                            Ok(DirMovement::Root) => { return Ok(DirMovement::Root); },

                            // For Up command, the single return is enough (continue processing)
                            Ok(DirMovement::Up) => { },

                            // For EOF, continue returning
                            Ok(DirMovement::EOF) => { return Ok(DirMovement::EOF); },

                            _ => { }
                        }
                    },
                },

                // ls doesn't need to do much, but could be useful to indicate when a directory
                // starts
                Some("ls") => {
                    println!("Listing {}", full_path);
                }

                // all other commands are invalid
                _ => { },
            },

            // a word 'dir' indicates that the following word is the name of a directory
            Some("dir") => {
                // create the directory
                let new_dir_name = words.next().unwrap();
                println!("new dir: {}", new_dir_name);

                let new_dir_str = new_dir_name.to_string();

                // create a new directory
                let new_dir = Directory {
                    name       : new_dir_str.clone(),
                    files      : HashMap::new(),
                    directories: HashMap::new(),
                    size       : 0
                };

                // let hashmap take ownership
                current_directory.directories.insert(new_dir_str, new_dir); 
            },

            // anything else should be a number and thus the size of a file
            sz @ _ => {
                let file_size: u32 = sz.unwrap().parse().expect("must be an int");
                let file_name_str = words.next().unwrap().to_string();
                println!("file {} has size {}", file_name_str, file_size);

                // create the file
                let new_file = File {
                    name: file_name_str.clone(),
                    size: file_size
                };

                // insert the file into the directory
                current_directory.files.insert(file_name_str, new_file);
            },
        }
    }
}

fn compute_sizes(current_directory: &mut Directory) -> u32 {
    let mut total_size = 0u32;

    // compute all the sizes of the sub directories
    for sub_directory in current_directory.directories.values_mut() {
        total_size += compute_sizes(sub_directory);
    }

    // sum up the file sizes too
    for file in current_directory.files.values() {
        total_size += file.size;
    }

    println!("directory {} has total size {}", current_directory.name, total_size);
    current_directory.size = total_size;
    total_size
}

fn get_weird_sum(current_directory: &mut Directory, limit: u32) -> u32 {
    let mut total = 0u32;

    // compute all the sizes of the sub directories
    for sub_directory in current_directory.directories.values_mut() {
        total += get_weird_sum(sub_directory, limit);
    }

    if current_directory.size < limit {
        current_directory.size + total
    } else {
        total
    }
}

// find the directory that frees at least need_to_free but as little over as possible
fn min_to_free(current_directory: &mut Directory, need_to_free: u32) -> Option<u32> {
    let mut goal = None;

    // find the minimum of all the subdirs that satisfy the critita
    for sub_directory in current_directory.directories.values_mut() {
        goal = match min_to_free(sub_directory, need_to_free) {
            Some(x) => if goal.is_none() || x < goal.unwrap() { Some(x) } else { goal },
            None    => goal,
        };
    }

    if current_directory.size >= need_to_free && (goal.is_none() || current_directory.size < goal.unwrap()) {
        Some(current_directory.size)
    } else {
        goal
    }
}


fn main() {
    // build the root directory structure
    let mut root_directory = Directory { 
        name       : "/".to_string(), 
        directories: HashMap::new(),
        files      : HashMap::new(),
        size       : 0,
    };

    // read in the file and create a line parser
    let file = std::fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let mut lines = file.split("\r\n").filter(|&s| s.len() > 0);

    // loop forever, restarting in the root (we should only get here with a 
    // cd / or a cd .. from the root)
    loop {
        // execute commands as if we're in the root directory
        match parse_commands(root_directory.name.clone(), &mut root_directory, &mut lines) {
            Ok(DirMovement::Root) => { println!("parse_commands exited for Root"); },
            Ok(DirMovement::Up)   => { println!("parse_commands exited for Up"); },
            Ok(DirMovement::EOF) => { break; },
            // other errors
            _ => { break; },
        }
    }

    let total_size = compute_sizes(&mut root_directory);
    println!("total size of filesystem = {}", total_size);

    let weird_sum = get_weird_sum(&mut root_directory, 100000);
    println!("weird sum (day7a) = {}", weird_sum);

    let to_free = min_to_free(&mut root_directory, 30000000 - (70000000 - total_size));
    println!("to free (day7b) = {}", to_free.unwrap());
}
