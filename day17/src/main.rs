use std::collections::HashSet;
use std::{env,process,fs};

const MAP_WIDTH: u8 = 7;
const X_OFFSET: u8 = 2;
const Y_OFFSET: u64 = 3;
const DEF_NUM_ROCKS: u64 = 2022;

// Kinds of rocks that can fall
// 1- Line:        @###
//
//                  #
// 2- Plus:        @##
//                  #
//
// 3- L (inverted):  #
//                   #
//                 @##
//
// 4- I:           #
//                 #
//                 #
//                 @
//
// 5- Block:       ##
//                 @#

#[derive(PartialEq)]
enum Rock {
    Line,
    Plus,
    L,
    I,
    Block,
}

impl Rock {
    fn neighbours_left(&self, pos: &(u8, u64)) -> Vec<(u8,u64)> {
        let mut v = Vec::new();
        let bl = pos.0 > 0;
        if bl {
            v.push((pos.0 - 1, pos.1));
        }
        match self {
            Self::Line => {},
            Self::Plus => {
                v.push((pos.0, pos.1 - 1));
                v.push((pos.0, pos.1 + 1));
            },
            Self::L => {
                v.push((pos.0 + 1, pos.1 + 1));
                v.push((pos.0 + 1, pos.1 + 2));
            },
            Self::I => {
                if bl {
                    for i in 1..4 {
                        v.push((pos.0 - 1, pos.1 + i));
                    }

                }
            },
            Self::Block => {
                if bl {
                    v.push((pos.0 - 1, pos.1 + 1));
                }
            },
        }
        v
    }

    fn neighbours_right(&self, pos: &(u8, u64)) -> Vec<(u8,u64)> {
        let mut v = Vec::new();
        match self {
            Self::Line => v.push((pos.0 + 4, pos.1)),
            Self::Plus => {
                v.push((pos.0 + 3, pos.1));
                v.push((pos.0 + 2, pos.1 - 1));
                v.push((pos.0 + 2, pos.1 + 1));
            },
            Self::L => {
                for i in 0..3 {
                    v.push((pos.0 + 3, pos.1 + i));
                }
            },
            Self::I => {
                for i in 0..4 {
                    v.push((pos.0 + 1, pos.1 + i));
                }

            },
            Self::Block => {
                v.push((pos.0 + 2, pos.1));
                v.push((pos.0 + 2, pos.1 + 1));
            },
        }
        v
    }

    fn neighbours_down(&self, pos: &(u8, u64)) -> Vec<(u8,u64)> {
        let mut v = Vec::new();
        if pos.1 > 0 {
            match self {
                Self::Line => {
                    for i in 0..4 {
                        v.push((pos.0 + i, pos.1 - 1));
                    }
                },
                Self::Plus => {
                    v.push((pos.0, pos.1 - 1));
                    v.push((pos.0 + 2, pos.1 - 1));
                    if pos.1 > 1 {
                        v.push((pos.0 + 1, pos.1 - 2));
                    }
                },
                Self::L => {
                    for i in 0..3 {
                        v.push((pos.0 + i, pos.1 - 1));
                    }
                },
                Self::I => v.push((pos.0, pos.1 - 1)),
                Self::Block => {
                    v.push((pos.0, pos.1 - 1));
                    v.push((pos.0 + 1, pos.1 - 1));
                },
            }
        }
        v
    }

    fn coordinates(&self, pos: &(u8, u64)) -> Vec<(u8, u64)> {
        let mut v = Vec::new();
        match self {
            Self::Line => {
                for i in 0..4 {
                    v.push((pos.0 + i, pos.1));
                }
            },
            Self::Plus => {
                for i in 0..3 {
                    v.push((pos.0 + i, pos.1));
                }
                v.push((pos.0 + 1, pos.1 - 1));
                v.push((pos.0 + 1, pos.1 + 1));
            },
            Self::L => {
                for i in 0..3 {
                    v.push((pos.0 + i, pos.1));
                }
                v.push((pos.0 + 2, pos.1 + 1));
                v.push((pos.0 + 2, pos.1 + 2));
            },
            Self::I => {
                for i in 0..4 {
                    v.push((pos.0, pos.1 + i));
                }
            },
            Self::Block => {
                for i in 0..2 {
                    for j in 0..2 {
                        v.push((pos.0 + i, pos.1 + j));
                    }
                }
            },
        }
        v
    }
}

fn get_next_falling_rock(num: u64) -> Rock {
    use Rock::*;
    match num % 5 {
        0 => Line,
        1 => Plus,
        2 => L,
        3 => I,
        4 => Block,
        _ => {
            panic!("Unreachable");
        },
    }
}

struct Cave {
    height: u64,
    rocks: HashSet<(u8,u64)>,
}

impl Cave {
    fn new() -> Self {
        Self {
            height: 0,
            rocks: HashSet::new()
        }
    }

    fn collides(&self, rock: &Rock, pos: &(u8,u64)) -> bool {
        if pos.1 == 0 || (rock == &Rock::Plus && pos.1 == 1){
            return true;
        }
        let mut blocked = false;
        for n in rock.neighbours_down(pos) {
            blocked |= self.rocks.contains(&n);
        }
        blocked
    }

    fn update_height(&mut self, h: u64) {
        if h + 1 > self.height {
            self.height = h + 1;
        }
    }

    fn rest(&mut self, rock: &Rock, pos: &(u8,u64)) {
        for r in rock.coordinates(pos) {
            self.rocks.insert(r);
            self.update_height(r.1);
        };
    }

    fn move_left(&self, rock: &Rock, pos: &(u8, u64)) -> (u8, u64) {
        if pos.0 == 0 {
            return (pos.0, pos.1);
        }

        let neighs_left = rock.neighbours_left(pos);
        let mut blocked = false;
        for n in neighs_left {
            blocked |= self.rocks.contains(&n);
        }

        match blocked {
            true => (pos.0, pos.1),
            false => (pos.0 - 1, pos.1),
        }
    }

    fn move_right(&self, rock: &Rock, pos: &(u8, u64)) -> (u8, u64) {
        let mut blocked = false;
        for n in rock.neighbours_right(pos) {
            blocked |= n.0 >= MAP_WIDTH;
            blocked |= self.rocks.contains(&n);
        }
        match blocked {
            true => (pos.0, pos.1),
            false => (pos.0 + 1, pos.1),
        }
    }

    // For testing purposes
    fn draw(&self) {
        let mut y = self.height;
        loop {
            let mut line = String::new();
            line.push_str(&(y % 10).to_string());
            line.push('|');
            for x in 0..MAP_WIDTH {
                if self.rocks.contains(&(x,y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            line.push('|');
            println!("{line}");
            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }
        println!(" +-------+");
    }
}

// Returns the type of rock that fell, its x-position (between 0 and MAP_WIDTH) and the height.
fn drop_rock(cave: &mut Cave, counter: u64, jets: &mut dyn Iterator<Item = char>) -> (Rock, (u8, u64)) {
    let rock = get_next_falling_rock(counter);
    let mut pos: (u8, u64);
    //Take into account the + shape
    if rock == Rock::Plus {
        pos = (X_OFFSET, cave.height + Y_OFFSET + 1);
    } else {
        pos = (X_OFFSET, cave.height + Y_OFFSET);
    }

    loop {
        //Move left/right
        pos = match jets.next() {
            Some('<') => cave.move_left(&rock,&pos),
            Some('>') => cave.move_right(&rock,&pos),
            Some(c) => {
                panic!("Unexpected character on input: {c}");
            },
            None => {
                panic!("Input finished unexpectedly");
            },
        };

        // Move down
        if cave.collides(&rock, &pos) {
            cave.rest(&rock, &pos);
            //println!("({},{})",pos.0,pos.1);
            return (rock, (pos.0, cave.height));
        } else {
            pos = (pos.0, pos.1 - 1);
        }
    }
}

fn run1(input: &str, lim: u64, print_percent: bool) -> u64 {
    let percent = print_percent && lim > 10000;
    let step = lim / 10000;
    let mut jets = input.chars().cycle();

    let mut cave = Cave::new();

    for i in 0..lim {
        if percent && i % step == 0 {
            let prct = i / step;
            println!("{}%", (prct as f64) / 100.0);
        }

        drop_rock(&mut cave, i, &mut jets);
        //cave.draw();
        //println!("");
    }

    cave.height
}

fn run2(input: &str, lim: u64, print_percent: bool) -> u64 {
    let percent = print_percent && lim > 10000;
    let step = lim / 10000;

    let mut jets = input.chars().cycle();

    let mut cave = Cave::new();
    let mut tracking = Vec::new();

    let mut i = 0;
    while i < lim {
        if percent && i % step == 0 {
            let prct = i / step;
            println!("{}%", (prct as f64) / 100.0);
        }

        let track = drop_rock(&mut cave, i, &mut jets);
        tracking.push(track);

        // Check for cycles
        if let Some(h) = find_period(&tracking,lim) {
            return h;
        }
        i += 1;
    }

    cave.height
}

fn find_period(heights: &Vec<(Rock, (u8, u64))>, total_rocks: u64) -> Option<u64> {
    for period in (5..(heights.len() / 2)).step_by(5) {
        // Separate heights in three parts: | remainder | period1 | period2 |
        // Here period1 and period2 have length period, and therefore remainder has the length
        // computed below
        let remainder_length = heights.len() - 2*period;
        let (remainder, periods) = heights.split_at(remainder_length);
        let (period1, period2) = periods.split_at(period);
        let offset1 = *remainder.last().map(|(_,(_, h))| h).unwrap_or(&0);
        let offset2 = *period1.last().map(|(_,(_,h))| h).expect("period1 should have something!");

        let is_period = period1
            .iter()
            .zip(period2.iter())    // Join iterators into an iterator of pairs
            .all(|((r1, (x1, h1)), (r2, (x2, h2)))| {
                *r1 == *r2 && *x1 == *x2 && h1 - offset1 == h2 - offset2
            });

        if is_period {
            let height_period = period1.last().map(|(_,(_,h))| h).unwrap() - offset1;
            let periodic_length = total_rocks - (remainder_length as u64);
            let num_periods_left = periodic_length / (period as u64);
            let rocks_left = (periodic_length as usize) % period;

            //This includes the height from the start of the sequence
            let rocks_left_height = match rocks_left > 0 {
                true => *period1.get(rocks_left - 1).map(|(_, (_, h))| h).unwrap(),
                false => offset1,
            };
            let total_height = rocks_left_height + (height_period * num_periods_left);

            return Some(total_height);
        }
    }
    None
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

    let lim;
    if let Some(s) = args.next() {
        lim = s.parse().unwrap();
    } else {
        lim = DEF_NUM_ROCKS;
    }

    let temp = fs::read_to_string(filepath).unwrap();
    let input = temp.trim();
    println!("{}", run2(input, lim, false));
}

#[test]
fn example1() {
    let temp = fs::read_to_string("test.txt").unwrap();
    let input = temp.trim();
    let n = run1(input, 2022, false);
    assert_eq!(n, 3068);
}

#[test]
fn input1() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let input = temp.trim();
    let n = run1(input, 2022, false);
    assert_eq!(n, 3181);
}

#[test]
fn example2() {
    let temp = fs::read_to_string("test.txt").unwrap();
    let input = temp.trim();
    let n = run2(input, 1000000000000, false);
    assert_eq!(n, 1514285714288);
}

#[test]
fn input2() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let input = temp.trim();
    let n = run2(input, 1000000000000, false);
    assert_eq!(n,1570434782634);
}
