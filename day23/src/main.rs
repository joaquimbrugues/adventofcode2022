use std::{env,fs,process};
use std::collections::{HashSet,HashMap};

#[derive(Clone, Copy, Debug)]
enum Dir { North, South, West, East, }

impl Dir {
    fn enumerate() -> Vec<Self> {
        vec![Dir::North, Dir::South, Dir::West, Dir::East]
    }
}

fn has_neighbours(pos: &(i32,i32), occupied: &HashSet<(i32,i32)>) -> bool {
    let mut has = false;
    has |= occupied.contains(&(pos.0 - 1, pos.1));
    has |= occupied.contains(&(pos.0 - 1, pos.1 + 1));
    has |= occupied.contains(&(pos.0, pos.1 + 1));
    has |= occupied.contains(&(pos.0 + 1, pos.1 + 1));
    has |= occupied.contains(&(pos.0 + 1, pos.1));
    has |= occupied.contains(&(pos.0 + 1, pos.1 - 1));
    has |= occupied.contains(&(pos.0, pos.1 - 1));
    has |= occupied.contains(&(pos.0 - 1, pos.1 - 1));
    has
}

fn area_min_rectangle(elves: &HashSet<(i32,i32)>) -> usize {
    let mut max_west = 0;
    let mut max_north = 0;
    let mut max_east = 0;
    let mut max_south = 0;
    let mut first = true;
    for &(y,x) in elves {
        if first || y > max_south {
            max_south = y;
        }
        if first || y < max_north {
            max_north = y;
        }
        if first || x > max_east {
            max_east = x;
        }
        if first || x < max_west {
            max_west = x;
        }
        first = false;
    }

    ((max_south - max_north + 1) * (max_east - max_west + 1)) as usize
}

fn print_elves(elves: &HashSet<(i32,i32)>) {
    let mut max_west = 0;
    let mut max_north = 0;
    let mut max_east = 0;
    let mut max_south = 0;
    let mut first = true;
    for &(y,x) in elves {
        if first || y > max_south {
            max_south = y;
        }
        if first || y < max_north {
            max_north = y;
        }
        if first || x > max_east {
            max_east = x;
        }
        if first || x < max_west {
            max_west = x;
        }
        first = false;
    }

    for y in max_north..=max_south {
        let mut line = String::from("");
        for x in max_west..=max_east {
            if elves.contains(&(y,x)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }
}

fn run1(input: &str) -> usize {
    // Read input
    let mut i = 0;
    let mut elves = HashSet::new();
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            if c == '#' {
                elves.insert((i,j));
            }
            j += 1;
        }
        i += 1;
    }
    // Execute 10 rounds
    let num_rounds = 10;
    for round in 0..num_rounds {
        // For each elf, insert their proposed movement in the map
        let mut movements: HashMap<(i32,i32), Vec<(i32,i32)>> = HashMap::with_capacity(elves.len());
        for elf in &elves {
            if has_neighbours(elf, &elves) {
                let directions = Dir::enumerate();
                for d in 0..4 {
                    let dir = directions[(d + round) % 4];
                    match dir {
                        Dir::North => {
                            if !(elves.contains(&(elf.0 - 1, elf.1 - 1)) || elves.contains(&(elf.0 - 1, elf.1)) || elves.contains(&(elf.0 - 1, elf.1 + 1))) {
                                if movements.contains_key(&(elf.0 - 1, elf.1)) {
                                    movements.get_mut(&(elf.0 - 1, elf.1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0 - 1, elf.1), vec![*elf]);
                                }
                                break;
                            }
                        },
                        Dir::South => {
                            if !(elves.contains(&(elf.0 + 1, elf.1 - 1)) || elves.contains(&(elf.0 + 1, elf.1)) || elves.contains(&(elf.0 + 1, elf.1 + 1))) {
                                if movements.contains_key(&(elf.0 + 1, elf.1)) {
                                    movements.get_mut(&(elf.0 + 1, elf.1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0 + 1, elf.1), vec![*elf]);
                                }
                                break;
                            }
                        },
                        Dir::West => {
                            if !(elves.contains(&(elf.0 - 1, elf.1 - 1)) || elves.contains(&(elf.0, elf.1 - 1)) || elves.contains(&(elf.0 + 1, elf.1 - 1))) {
                                if movements.contains_key(&(elf.0, elf.1 - 1)) {
                                    movements.get_mut(&(elf.0, elf.1 - 1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0, elf.1 - 1), vec![*elf]);
                                }
                                break;
                            }
                        },
                        Dir::East => {
                            if !(elves.contains(&(elf.0 - 1, elf.1 + 1)) || elves.contains(&(elf.0, elf.1 + 1)) || elves.contains(&(elf.0 + 1, elf.1 + 1))) {
                                if movements.contains_key(&(elf.0, elf.1 + 1)) {
                                    movements.get_mut(&(elf.0, elf.1 + 1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0, elf.1 + 1), vec![*elf]);
                                }
                                break;
                            }
                        },
                    }
                }
            }
        }
        
        // Attempt to execute proposed movements
        for (pos, v) in movements {
            // Execute only if only one elf wants to get to the position pos
            if v.len() == 1 {
                elves.remove(&v[0]);
                elves.insert(pos);
            }
        }
    }

    // Count free spots in the minimal rectangle containing all the elves
    area_min_rectangle(&elves) - elves.len()
}

fn run2(input: &str) -> usize {
    // Read input
    let mut i = 0;
    let mut elves = HashSet::new();
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            if c == '#' {
                elves.insert((i,j));
            }
            j += 1;
        }
        i += 1;
    }
    let mut round = 0;
    loop {
        // For each elf, insert their proposed movement in the map
        let mut movements: HashMap<(i32,i32), Vec<(i32,i32)>> = HashMap::with_capacity(elves.len());
        for elf in &elves {
            if has_neighbours(elf, &elves) {
                let directions = Dir::enumerate();
                for d in 0..4 {
                    let dir = directions[(d + round) % 4];
                    match dir {
                        Dir::North => {
                            if !(elves.contains(&(elf.0 - 1, elf.1 - 1)) || elves.contains(&(elf.0 - 1, elf.1)) || elves.contains(&(elf.0 - 1, elf.1 + 1))) {
                                if movements.contains_key(&(elf.0 - 1, elf.1)) {
                                    movements.get_mut(&(elf.0 - 1, elf.1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0 - 1, elf.1), vec![*elf]);
                                }
                                break;
                            }
                        },
                        Dir::South => {
                            if !(elves.contains(&(elf.0 + 1, elf.1 - 1)) || elves.contains(&(elf.0 + 1, elf.1)) || elves.contains(&(elf.0 + 1, elf.1 + 1))) {
                                if movements.contains_key(&(elf.0 + 1, elf.1)) {
                                    movements.get_mut(&(elf.0 + 1, elf.1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0 + 1, elf.1), vec![*elf]);
                                }
                                break;
                            }
                        },
                        Dir::West => {
                            if !(elves.contains(&(elf.0 - 1, elf.1 - 1)) || elves.contains(&(elf.0, elf.1 - 1)) || elves.contains(&(elf.0 + 1, elf.1 - 1))) {
                                if movements.contains_key(&(elf.0, elf.1 - 1)) {
                                    movements.get_mut(&(elf.0, elf.1 - 1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0, elf.1 - 1), vec![*elf]);
                                }
                                break;
                            }
                        },
                        Dir::East => {
                            if !(elves.contains(&(elf.0 - 1, elf.1 + 1)) || elves.contains(&(elf.0, elf.1 + 1)) || elves.contains(&(elf.0 + 1, elf.1 + 1))) {
                                if movements.contains_key(&(elf.0, elf.1 + 1)) {
                                    movements.get_mut(&(elf.0, elf.1 + 1)).unwrap().push(*elf);
                                } else {
                                    movements.insert((elf.0, elf.1 + 1), vec![*elf]);
                                }
                                break;
                            }
                        },
                    }
                }
            }
        }
        round += 1;
        
        // If no one moves, return the round!
        if movements.len() == 0 {
            // Print the map
            print_elves(&elves);
            return round;
        }

        // Attempt to execute proposed movements
        for (pos, v) in movements {
            // Execute only if only one elf wants to get to the position pos
            if v.len() == 1 {
                elves.remove(&v[0]);
                elves.insert(pos);
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,110);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,4241);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,20);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1079);
}
