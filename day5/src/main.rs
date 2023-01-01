use std::{env,fs,process};
use std::collections::VecDeque;

fn run1(input: &str) -> String {
    let mut stacks = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.contains(|c: char| c.is_ascii_digit()) {
            break;
        }
        let crates: Vec<&str> = line.split(' ').collect();
        // Initialize stacks
        if stacks.len() == 0 {
            for _ in 0..crates.len() {
                stacks.push(VecDeque::new());
            }
        }

        // Read crates
        let mut i = 0;
        for c in crates {
            let c = c.trim();
            let id = c.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            if id != "-" {
                stacks[i].push_front(id);
            }
            i += 1;
        }
    }
    // Skip blank line
    lines.next();
    
    // Parse movements
    while let Some(line) = lines.next() {
        // "move X from P1 to P2"
        let mut words = line.split(' ');
        words.next();
        let n = words.next().unwrap().trim().parse::<usize>().unwrap();
        words.next();
        let p1 = words.next().unwrap().trim().parse::<usize>().unwrap() - 1;
        words.next();
        let p2 = words.next().unwrap().trim().parse::<usize>().unwrap() - 1;
        for _ in 0..n {
            let id = stacks[p1].pop_back().unwrap();
            stacks[p2].push_back(id);
        }
    }

    // Combine result
    let mut res = String::from("");
    for mut s in stacks {
        res.push_str(s.pop_back().unwrap());
    }
    res
}

fn run2(input: &str) -> String {
    let mut stacks = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.contains(|c: char| c.is_ascii_digit()) {
            break;
        }
        let crates: Vec<&str> = line.split(' ').collect();
        // Initialize stacks
        if stacks.len() == 0 {
            for _ in 0..crates.len() {
                stacks.push(VecDeque::new());
            }
        }

        // Read crates
        let mut i = 0;
        for c in crates {
            let c = c.trim();
            let id = c.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            if id != "-" {
                stacks[i].push_front(id);
            }
            i += 1;
        }
    }
    // Skip blank line
    lines.next();
    
    // Parse movements
    while let Some(line) = lines.next() {
        // "move X from P1 to P2"
        let mut words = line.split(' ');
        words.next();
        let n = words.next().unwrap().trim().parse::<usize>().unwrap();
        words.next();
        let p1 = words.next().unwrap().trim().parse::<usize>().unwrap() - 1;
        words.next();
        let p2 = words.next().unwrap().trim().parse::<usize>().unwrap() - 1;
        for i in 0..n {
            let l = stacks[p1].len();
            let id = stacks[p1].remove(l - (n - i)).unwrap();
            stacks[p2].push_back(id);
        }
    }

    // Combine result
    let mut res = String::from("");
    for mut s in stacks {
        res.push_str(s.pop_back().unwrap());
    }
    res
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
    assert_eq!(res.as_str(),"CMZ");
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res.as_str(),"CFFHVVHNC");
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res.as_str(),"MCD");
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res.as_str(),"FSZWBPTBG");
}
