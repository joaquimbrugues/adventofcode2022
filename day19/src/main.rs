use std::{env,fs,process};
use std::collections::HashSet;

// Keys:
// 0 - Ore
// 1 - Clay
// 2 - Obsidian
// 3 - Geode

struct Blueprint {
    prices: [[u32;3];4],
    //min_cost: [u32;3],
    max_cost: [u32;3],
}

impl Blueprint {
    fn res_from_word(word: &str) -> usize {
        match word {
            "ore" => 0,
            "clay" => 1,
            "obsidian" => 2,
            "geode" => 3,
            _ => {
                panic!("Unexpected input");
            }
        }
    }

    fn parse_blueprint(line: &str) -> Self {
        let mut max_cost = [0;3];
        let (_, contents) = line.split_once(':').unwrap();
        let mut prices = [[0;3];4];
        for robotstr in contents.split('.') {
            if robotstr.len() > 0 {
                // Each {resource} robot costs XX {resource} (and XX {resource})*
                let robotstr = robotstr.trim();
                let mut words = robotstr.split(' ');
                words.next();
                let robot_index = Self::res_from_word(words.next().unwrap());
                words.next();
                words.next();
                let mut price = [0;3];
                while let Some(mut s) = words.next() {
                    if s == "and" {
                        s = words.next().unwrap();
                    }
                    let cost = s.parse::<u32>().unwrap();
                    let res_ind = Self::res_from_word(words.next().unwrap());
                    price[res_ind] = cost;
                    if cost > max_cost[res_ind] {
                        max_cost[res_ind] = cost;
                    }
                }
                prices[robot_index] = price;
            }
        }
        Blueprint{
            prices,
            max_cost,
        }
    }

    // Returns None if the robot cannot be build at all with the current production
    // Otherwise, returns Some(time), where time is the minimum number of minutes required to gather the
    // resources required for this robot.
    fn time_until_can_build(&self, resources: &[u32], robots: &[u32; 4], remaining_time: u32, robot: usize) -> Option<u32> {
        // OPTIMIZATION 1:
        // Return None if we already have the maximum number of this type that we could need
        if robot < 3 && (remaining_time * robots[robot]) + resources[robot] >= remaining_time * self.max_cost[robot] {
            return None;
        }
        let mut time = 0;
        for r in 0..3 {
            // Check if we already have the needed resources
            if resources[r] < self.prices[robot][r] {
                // We need to gather resources.
                // Check if we have the required robot
                if robots[r] == 0 {
                    return None;
                }
                // This is Euclidean division. We must round the result up, thus the modulo
                // operation
                let mut t = (self.prices[robot][r] - resources[r]) / robots[r];
                if (self.prices[robot][r] - resources[r]) % robots[r] != 0 {
                    t += 1;
                }
                if t > time {
                    time = t;
                }
            }
        }
        Some(time)
    }
}

fn add<const N: usize>(a: [u32;N], b: [u32;N]) -> [u32;N] {
    let mut c = [0;N];
    for i in 0..N {
        c[i] = a[i] + b[i];
    }
    c
}

fn substract<const M: usize,const N: usize>(a: [u32;M], b: [u32;N]) -> [u32;M] {
    if M < N {
        panic!("First array should be longer or equal to the second");
    }
    let mut c = [0;M];
    for i in 0..N {
        if a[i] < b[i] {
            panic!("Substraction not possible: {a:?} - {b:?}");
        }
        c[i] = a[i] - b[i];
    }
    for i in N..M {
        c[i] = a[i];
    }
    c
}

fn product<const N: usize>(a: u32, b: [u32;N]) -> [u32;N] {
    let mut c = [0;N];
    for i in 0..N {
        c[i] = a * b[i];
    }
    c
}

// Return 1 + 2 + 3 + 4 +···+ n
fn triangular(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

fn max_geodes(blueprint: &Blueprint, resources: [u32;4], robots: [u32;4], minutes: u32) -> (u32, Vec<String>) {
    // Depth-first search scheme
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut max = (0, Vec::new()); // Second component is the trace up to this point
    stack.push((resources, robots, 0, vec![format!("Min: 0, bots: {robots:?}, resources: {resources:?}")]));

    while let Some((resources, robots, minute, trace)) = stack.pop() {
        // We are at the beginning of minute {minute}

        if !visited.contains(&(resources, robots, minute)) {
            visited.insert((resources, robots, minute));

            // If we do nothing and just harvest geodes:
            let g = resources[3] + (robots[3] * (minutes - minute));
            if g > max.0 {
                max = (g, trace.clone());
            }
            
            if minute < minutes - 1 {

                // OPTIMIZATION 2: try to prune branches
                // Be very generous and imagine that we produce a new geode cracker every minute
                // after now
                if g + triangular(minutes - minute - 1) < max.0 {
                    continue;
                }

                // Find "neighbours": for each type of robot, compute the time until we have the
                // required resources to build it


                let options = (0..4).map(|o| (o,blueprint.time_until_can_build(&resources[0..3],&robots, minutes - minute,o)))
                    .filter(|(_,s)| s.is_some())
                    .map(|(o,s)| (o,s.unwrap()));
                for (o, t) in options {
                    if minute + t + 1 < minutes {
                        //Compute data for next node in this branch
                        let nresources = substract(add(resources, product(t+1, robots)), blueprint.prices[o]);
                        let mut nrobots = robots;
                        nrobots[o] += 1;
                        let ntime = minute + t + 1;
                        let mut ntrace = trace.clone();
                        ntrace.push(format!("Min: {}, bots: {nrobots:?}, resources: {nresources:?}", minute + t + 1));
                        stack.push((nresources, nrobots, ntime, ntrace));
                    }
                }
            }
        }
    }
    max
}

fn run1(input: &str) -> u32 {
    let mut i = 1;
    let mut sum = 0;
    // Parse input
    for line in input.lines() {
        if line.starts_with("//") {
            continue;
        }
        let blueprint = Blueprint::parse_blueprint(line);

        // Initializations
        let resources = [0;4];
        let robots = [1, 0, 0, 0];
        let g = max_geodes(&blueprint, resources, robots, 24);
        sum += i * g.0;
        i += 1;
    }
    sum
}

fn run2(input: &str) -> u32 {
    let mut i = 0;
    let mut prod = 1;
    for line in input.lines() {
        if i >= 3 {
            break;
        }
        if line.starts_with("//") {
            continue;
        }
        let blueprint = Blueprint::parse_blueprint(line);

        // Initializations
        let resources = [0;4];
        let robots = [1, 0, 0, 0];
        let g = max_geodes(&blueprint, resources, robots, 32);
        prod *= g.0;
        i += 1;
    }
    prod
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
    assert_eq!(res,33);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1389);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3472);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3003);
}
