use std::{env,fs,process};
use std::collections::{HashSet,HashMap,VecDeque,BinaryHeap};
use std::cell::RefCell;

const INIT: &str = "AA";

struct Valve<'a> {
    rate: u32,
    neighbours: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    // Format:
    // Valve XY has flow rate={rate}; tunnels lead to valve(s) XZ, XW, YZ
    fn read_line(line: &'a str) -> (&'a str, Self) {
        let mut words = line.split(' ');
        words.next();
        let id = words.next().unwrap();
        words.next();
        words.next();
        let rate = words.next().unwrap()
            .strip_prefix("rate=").unwrap()
            .strip_suffix(';').unwrap()
            .parse::<u32>().unwrap();
        for _i in 0..4 {
            words.next();
        }
        let mut neighbours = Vec::new();
        while let Some(s) = words.next() {
            let nid = s.strip_suffix(',').unwrap_or(s);
            neighbours.push(nid);
        }
        (id, Valve { rate, neighbours })
    }
}

struct Graph<'a> {
    nodes: HashMap<&'a str, (usize, Valve<'a>)>,
    distances: RefCell<HashMap<(&'a str, &'a str), u32>>,
    interesting_valves: HashSet<&'a str>,
    min_dist: RefCell<Option<u32>>,
}

impl<'a> Graph<'a> {
    fn read_graph(input: &'a str) -> Self {
        let mut graph = HashMap::new();
        let mut interesting = HashSet::new();
        let mut i = 0;
        for line in input.lines() {
            let (id, v) = Valve::read_line(line);
            if v.rate > 0 {
                interesting.insert(id);
            }
            graph.insert(id, (i, v));
            i += 1;
        }
        Self {
            nodes: graph,
            distances: RefCell::new(HashMap::new()),
            interesting_valves: interesting,
            min_dist: RefCell::new(None),
        }
    }

    fn distance(&'a self, id1: &'a str, id2: &'a str) -> u32 {
        let mut distances = self.distances.borrow_mut();
        match distances.get(&(id1,id2)) {
            Some(d) => *d,
            None => {
                // Breadth-first search
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                visited.insert(id1);
                queue.push_back((id1,0));
                while let Some((id,dist)) = queue.pop_front() {
                    if !distances.contains_key(&(id1,id)) {
                        distances.insert((id1,id),dist);
                        distances.insert((id,id1),dist);
                    }
                    if id == id2 {
                        return dist;
                    }
                    for n in &self.nodes.get(id).unwrap().1.neighbours {
                        if !visited.contains(n) {
                            visited.insert(n);
                            queue.push_back((n,dist + 1));
                        }
                    }
                }
                0
            },
        }
    }

    fn get_min_dist(&'a self) -> u32 {
        let interior = *self.min_dist.borrow();
        if interior.is_some() {
            return interior.unwrap();
        }

        let mut min = 0;
        for n1 in &self.interesting_valves {
            for n2 in &self.interesting_valves {
                if n1 != n2 {
                    let d = self.distance(n1,n2);
                    if min == 0 || d < min {
                        min = d;
                    } 
                }
            }
        }
        *self.min_dist.borrow_mut().insert(min)
    }
}

// Compute how much pressure we could release if we opened valve every minute from now on
fn optimistic_pressure<'a>(graph: &'a Graph<'a>, opened: &Vec<bool>, max_time: u32, interesting_valves: &HashSet<&str>) -> u32 {
    let mut heap = BinaryHeap::with_capacity(graph.interesting_valves.len());
    for idv in interesting_valves {
        let (index, valve) = graph.nodes.get(idv).unwrap();
        if !opened[*index] {
            heap.push(valve.rate);
        }
    }
    let mut pressure = 0;
    let d = graph.get_min_dist();
    let mut minutes = max_time;
    while minutes > d {
        minutes -= d;
        if let Some(p) = heap.pop() {
            pressure += p * minutes;
        } else {
            break;
        }
    }
    pressure
}

fn compute_pressure<'a>(graph: &'a Graph<'a>, id: &'a str, remaining_minutes: u32, interesting_valves: &HashSet<&'a str>) -> u32 {
    let mut max = 0;
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    // id, remaining_time, opened_valves, certain_pressure
    stack.push((id,remaining_minutes,0,0));
    while let Some((id, time, opened, pressure)) = stack.pop() {
        if !visited.contains(&(id,time,opened,pressure)) {
            visited.insert((id,time,opened,pressure));
            let opened_valves = vecb_from_int(opened, graph.nodes.len());
            // Optimization: trim the current branch if it is not good enough
            // Important: only trim past some point in the process
            if max > 0 /*&& 4 * time > remaining_minutes*/ {
                let optimistic = pressure + optimistic_pressure(graph, &opened_valves, time, interesting_valves);
                if optimistic <= max {
                    continue;
                }
            }
            for idv in interesting_valves {
                let (index, valve) = graph.nodes.get(idv).unwrap();
                let dist = graph.distance(id,idv);
                if !opened_valves[*index] && dist < time {
                    let mut opened_valves = opened_valves.clone();
                    // Travel to the valve idv and open it
                    opened_valves[*index] = true;
                    let mins = time - dist - 1;
                    let press = pressure + mins * valve.rate;
                    let nop = int_from_vecb(&opened_valves);
                    stack.push((idv, mins, nop, press));
                    if max < press {
                        max = press;
                    }
                }
            }
        }
    }
    max
}

fn run1(input: &str) -> u32 {
    let minutes = 30;
    let graph = Graph::read_graph(input);
    compute_pressure(&graph, INIT, minutes, &graph.interesting_valves)
}

fn int_from_vecb(vec: &Vec<bool>) -> u64 {
    let mut sum = 0;
    for (i, b) in vec.iter().enumerate() {
        if *b {
            sum += 2u64.pow(i as u32);
        }
    }
    sum
}

fn vecb_from_int(int: u64, length: usize) -> Vec<bool> {
    let mut v = vec![false;length];
    let mut i = 0;
    let mut tmp = int;
    while tmp > 0 {
        v[i] = match tmp % 2 {
            0 => false,
            1 => true,
            _ => panic!("Unreachable"),
        };
        tmp /= 2;
        i += 1;
    }
    v
}

fn divide_valves<'a>(set: &HashSet<&'a str>, length_diff: usize) -> Vec<(HashSet<&'a str>, HashSet<&'a str>)> {
    let mut v = Vec::new();
    let vector: Vec<&str> = set.iter().map(|&s| s).collect();
    let l = vector.len() as u32;
    let mut seen = HashSet::new();
    let max = 2usize.pow(l) - 1;
    for n in 1..max {
        let vb = vecb_from_int(n as u64,l as usize);
        let l1 = vb.iter().filter(|&b| *b).collect::<Vec<&bool>>().len();
        if ((l1 as isize) - ((l / 2) as isize)).abs() <= (length_diff as isize) && !seen.contains(&n) {
            // Mark this and its converse as seen
            seen.insert(n);
            seen.insert(max - n);
            // Include this
            let l0 = (l as usize) - l1;
            let mut s0 = HashSet::with_capacity(l0);
            let mut s1 = HashSet::with_capacity(l1);
            let mut i = 0;
            for b in vb {
                if b {
                    s1.insert(vector[i]);
                } else {
                    s0.insert(vector[i]);
                }
                i += 1;
            }
            v.push((s0,s1));
        }
    }
    v
}

fn run2(input: &str) -> u32 {
    let minutes = 26;
    let graph = Graph::read_graph(input);
    let mut max = 0;
    let options = divide_valves(&graph.interesting_valves,1);
    println!("{}", options.len());
    for (s0,s1) in options {
        let pr = compute_pressure(&graph, INIT, minutes, &s0) + compute_pressure(&graph, INIT, minutes, &s1);
        if max < pr {
            max = pr;
        }
    }
    max
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
    let res = run1(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let n = run1(&input);
    assert_eq!(n, 1651);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let n = run1(&input);
    assert_eq!(n, 1584);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let n = run2(&input);
    assert_eq!(n, 1707);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let n = run2(&input);
    assert_eq!(n, 2052);
}

#[test]
fn tint_from_vecb() {
    let v1 = vec![true,false,true,false,false];
    assert_eq!(int_from_vecb(&v1),5);
    let v2 = vec![false;20];
    assert_eq!(int_from_vecb(&v2),0);
    let v3 = vec![false,false,false,false,false,false,false,true,false];
    assert_eq!(int_from_vecb(&v3),128);
}

#[test]
fn tvecb_from_int() {
    assert_eq!(vecb_from_int(5,3),vec![true,false,true]);
    assert_eq!(vecb_from_int(0,3),vec![false,false,false]);
    assert_eq!(vecb_from_int(15,4),vec![true,true,true,true]);
    assert_eq!(vecb_from_int(15,5),vec![true,true,true,true,false]);
    assert_eq!(vecb_from_int(23,5),vec![true,true,true,false,true]);
}
