use std::{env,fs,process};
use std::collections::{HashMap,HashSet,VecDeque};

fn gcd(a: u32, b: u32)  -> u32 {
    let (mut a,mut b): (u32, u32) = (a,b);
    while b != 0 {
        (a,b) = (b, a % b);
    }
    a
}

fn mcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a,b)
}

#[derive(Clone,Copy,Debug)]
enum Dir { East, North, South, West }

struct Blizzard {
    map: HashMap<u32, Vec<(u32,u32,Dir)>>,
    height: u32,
    width: u32,
    period: u32,
}

impl Blizzard {
    fn read_blizzard(input: &str) -> (Vec<(u32,u32,Dir)>, u32, u32) {
        let mut i = 0;
        let mut j = 0;
        let mut v = Vec::new();
        for line in input.lines() {
            j = 0;
            for c in line.chars() {
                match c {
                    '#' | '.' => {}, // Do nothing
                    '<' => { // West
                        v.push((i,j,Dir::West));
                    },
                    '>' => { // East
                        v.push((i,j,Dir::East));
                    },
                    '^' => { // North
                        v.push((i,j,Dir::North));
                    },
                    'v' => { // South
                        v.push((i,j,Dir::South));
                    },
                    e => panic!("Unexpected character {e}"),
                }
                j += 1;
            }
            i += 1;
        }
        (v,i,j)
    }

    fn from_input(input: &str) -> Self {
        let (v,height,width) = Self::read_blizzard(input);
        let period = mcm(height,width);
        let mut map = HashMap::with_capacity(period as usize);
        map.insert(0,v);
        Self { map, height, width, period }
    }

    fn get_blizzard(&mut self, t: u32) -> &Vec<(u32,u32,Dir)> {
        let t = t % self.period;
        if !self.map.contains_key(&t) {
            let ov = self.map.get(&(t-1)).unwrap();
            let mut nv = Vec::with_capacity(ov.capacity());
            for &(i,j,dir) in ov {
                let (ni,nj,dir) = match dir {
                    Dir::East => {
                        if j == self.width - 2 {
                            (i,1,Dir::East)
                        } else {
                            (i, j+1, Dir::East)
                        }
                    },
                    Dir::West => {
                        if j == 1 {
                            (i, self.width - 2, Dir::West)
                        } else {
                            (i, j-1, Dir::West)
                        }
                    },
                    Dir::North => {
                        if i == 1 {
                            (self.height - 2, j, Dir::North)
                        } else {
                            (i-1, j, Dir::North)
                        }
                    },
                    Dir::South => {
                        if i == self.height - 2 {
                            (1,j,Dir::South)
                        } else {
                            (i+1, j, Dir::South)
                        }
                    },
                };
                nv.push((ni,nj,dir));
            }
            self.map.insert(t,nv);
        }
        self.map.get(&t).unwrap()
    }

    fn get_neighbours(&self, pos: (u32, u32), start: (u32,u32), end: (u32,u32)) -> Vec<(u32,u32)> {
        let mut v = vec![pos];
        if (pos.0 > 1 && pos.1 > 0 && pos.1 < self.width - 1) || (pos.0 > 0 && ((pos.0 - 1, pos.1) == start || (pos.0 - 1, pos.1) == end)) {
            v.push((pos.0 - 1, pos.1));
        }
        if (pos.0 < self.height - 2 && pos.1 > 0 && pos.1 < self.width - 1) || (pos.0 + 1, pos.1) == start || (pos.0 + 1, pos.1) == end {
            v.push((pos.0 + 1, pos.1));
        }
        if (pos.1 > 1 && pos.0 > 0 && pos.0 < self.height - 1) || (pos.1 > 0 && ((pos.0, pos.1 - 1) == start || (pos.0, pos.1 - 1) == end)) {
            v.push((pos.0, pos.1 - 1));
        }
        if (pos.1 < self.width - 2 && pos.0 > 0 && pos.0 < self.height - 1) || (pos.0, pos.1 + 1) == start || (pos.0, pos.1 + 1) == end {
            v.push((pos.0, pos.1 + 1));
        }
        v
    }

    fn in_blizzard(&mut self, pos: (u32,u32), t: u32) -> bool {
        for (i,j,_) in self.get_blizzard(t) {
            if pos == (*i,*j) {
                return true;
            }
        }
        false
    }

    fn print_map(&mut self, t: u32, pos: (u32, u32), start: (u32,u32), end: (u32,u32)) -> String {
        let height = self.height;
        let width = self.width;
        let bl = self.get_blizzard(t);
        let mut string = String::from("");
        for i in 0..height {
            for j in 0..width {
                if pos == (i,j) {
                    string.push('E');
                } else if start == (i,j) || end == (i,j) {
                    string.push('.');
                    
                } else if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
                    string.push('#');
                } else {
                    let mut c = String::from(".");
                    let mut num = 0;
                    for (y,x,dir) in bl {
                        if (*y,*x) == (i,j) {
                            if num == 0 {
                                c = match dir {
                                    Dir::East => String::from(">"),
                                    Dir::West => String::from("<"),
                                    Dir::North => String::from("^"),
                                    Dir::South => String::from("v"),
                                };
                            }
                            num += 1;
                        }
                    }
                    if num > 1 {
                        c = num.to_string();
                    }
                    string.push_str(&c);
                }
            }
            string.push('\n');
        }
        string.push('\n');
        string
    }
}

//fn find_path(blizzard: &mut Blizzard, start: (u32,u32), end: (u32,u32), start_time: u32, print_map: bool) -> u32 {
fn find_path(blizzard: &mut Blizzard, start: (u32,u32), end: (u32,u32), start_time: u32) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert((start,start_time));
    //queue.push_front((start,start_time, blizzard.print_map(start_time,start,start,end)));
    queue.push_front((start,start_time));
    //while let Some((pos,t,trace)) = queue.pop_back() {
    while let Some((pos,t)) = queue.pop_back() {
        //println!("{pos:?}");
        // Breadth-first search

        // Check if we are done
        if pos == end {
            //if print_map {
                //println!("{trace}");
            //}
            return t;
        }

        // If we are in the blizzard, discard this path
        if !blizzard.in_blizzard(pos, t) {
            for neigh in blizzard.get_neighbours(pos,start,end) {
                if !visited.contains(&(neigh,t+1)) {
                    visited.insert((neigh,t+1));
                    //let mut ntrace = trace.clone();
                    //ntrace.push_str(&blizzard.print_map(t+1,neigh,start,end));
                    //queue.push_front((neigh,t+1,ntrace));
                    queue.push_front((neigh,t+1));
                }
            }
        }
    }
    0
}

fn run1(input: &str) -> u32 {
    let mut blizzard = Blizzard::from_input(input);
    let start = (0,1);
    let end = (blizzard.height - 1, blizzard.width - 2);
    //find_path(&mut blizzard, start, end, 0, false)
    find_path(&mut blizzard, start, end, 0)
}

fn run2(input: &str) -> u32 {
    let mut blizzard = Blizzard::from_input(input);
    let start = (0,1);
    let end = (blizzard.height - 1, blizzard.width - 2);
    //let t1 = find_path(&mut blizzard, start, end, 0, false);
    //println!("{t1}");
    //let t2 = find_path(&mut blizzard, end, start, t1, true);
    //println!("{t2}");
    //let t3 = find_path(&mut blizzard, start, end, t2, false);
    //t3
    let t1 = find_path(&mut blizzard, start, end, 0);
    println!("{t1}");
    let t2 = find_path(&mut blizzard, end, start, t1);
    println!("{t2}");
    let t3 = find_path(&mut blizzard, start, end, t2);
    t3
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
    assert_eq!(res,18);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,247);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let mut blizzard = Blizzard::from_input(&input);
    let start = (0,1);
    let end = (blizzard.height - 1, blizzard.width - 2);
    //let t1 = find_path(&mut blizzard, start, end, 0,false);
    let t1 = find_path(&mut blizzard, start, end, 0);
    assert_eq!(t1,18);
    //let t2 = find_path(&mut blizzard, end, start, t1,false);
    let t2 = find_path(&mut blizzard, end, start, t1);
    assert_eq!(t2,41);
    //let t3 = find_path(&mut blizzard, start, end, t2,false);
    let t3 = find_path(&mut blizzard, start, end, t2);
    assert_eq!(t3,54);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut blizzard = Blizzard::from_input(&input);
    let start = (0,1);
    let end = (blizzard.height - 1, blizzard.width - 2);
    //let t1 = find_path(&mut blizzard, start, end, 0,false);
    let t1 = find_path(&mut blizzard, start, end, 0);
    assert_eq!(t1,247);
    //let t2 = find_path(&mut blizzard, end, start, t1,false);
    let t2 = find_path(&mut blizzard, end, start, t1);
    assert_eq!(t2,465);
    //let t3 = find_path(&mut blizzard, start, end, t2,false);
    let t3 = find_path(&mut blizzard, start, end, t2);
    assert_eq!(t3,728);
}
