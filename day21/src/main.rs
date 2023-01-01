use std::{env,fs,process};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug,Clone)]
enum Operation { Plus, Minus, Times, Div, Equals }

impl Operation {
    fn ex(&self, first: &u64, second: &u64) -> u64 {
        use Operation::*;
        match self {
            Plus => first + second,
            Minus => first - second,
            Times => first * second,
            Div => first / second,
            Equals => {
                panic!("This opeation is not implemented");
            },
        }
    }

    // Do inverse operation
    fn xe(&self, first: &u64, second: &u64) -> u64 {
        use Operation::*;
        match self {
            Plus => first - second,
            Minus => first + second,
            Times => first / second,
            Div => first * second,
            Equals => *second,
        }
    }

    fn parse(ch: &str) -> Self {
        use Operation::*;
        match ch {
            "+" => Plus,
            "-" => Minus,
            "*" => Times,
            "/" => Div,
            s => {
                panic!("Unexpected string {s}");
            },
        }
    }
}

struct Resolver<'a> {
    solved: RefCell<HashMap<&'a str, u64>>,
    equations: HashMap<&'a str, (&'a str, &'a str, Operation)>,
}

impl<'a> Resolver<'a> {
    fn new() -> Self {
        Resolver {
            solved: RefCell::new(HashMap::new()),
            equations: HashMap::new(),
        }
    }

    fn parse_line(&mut self, string: &'a str) {
        let (lhs,rhs) = string.split_once(':').unwrap();
        let (lhs,rhs) = (lhs.trim(),rhs.trim());
        let words: Vec<&str> = rhs.split(' ').collect();
        if words.len() == 1 {
            // Line with a number
            self.solved.borrow_mut().insert(lhs, words[0].parse().unwrap());
        } else {
            // Line with an operation
            let op = Operation::parse(words[1]);
            self.equations.insert(lhs, (words[0], words[2], op));
        }
    }

    fn resolve(&self, key: &'a str) -> u64 {
        if let Some(int) = self.solved.borrow().get(key) {
            return *int;
        }
        let (rhs1, rhs2, op) = self.equations.get(key).unwrap();
        let i1 = self.resolve(rhs1);
        let i2 = self.resolve(rhs2);
        let int = op.ex(&i1,&i2);
        self.solved.borrow_mut().insert(key,int);
        int
    }

    // Returns the path descending through the graph from {from} to {to}
    // Do not include {to} in the final list
    fn find(&'a self, from: &'a str, to: &'a str) -> Option<Vec<&'a str>> {
        if from == to {
            return Some(vec![]);
        }
        if let Some((rhs1, rhs2, _)) = self.equations.get(from) {
            if let Some(mut path) = self.find(rhs1, to) {
                path.push(from);
                return Some(path);
            }
            if let Some(mut path) = self.find(rhs2, to) {
                path.push(from);
                return Some(path);
            }
        }
        None
    }

    fn sibling(&'a self, parent: &str, child: &str) -> &'a str {
        let (r1, r2, _) = self.equations.get(parent).unwrap();
        if *r1 == child {
            return r2;
        } else if *r2 == child {
            return r1;
        } else {
            panic!("Node \"{parent}\" is not parent of node \"{child}\"");
        }
    }
}

fn run1(input: &str) -> u64 {
    let mut resolver = Resolver::new();
    for line in input.lines() {
        resolver.parse_line(line);
    }
    resolver.resolve("root")
}

fn run2(input: &str) -> u64 {
    let mut resolver = Resolver::new();
    for line in input.lines() {
        resolver.parse_line(line);
    }
    let mut tuple = resolver.equations.get_mut("root").unwrap();
    tuple.2 = Operation::Equals;
    let mut path = resolver.find("root", "humn").unwrap();
    let mut value = 0;
    while let Some(id) = path.pop() {
        let (r1, r2, op) = resolver.equations.get(id).unwrap();
        let next = match path.last() {
            Some(s) => s,
            None => "humn",
        };
        if next == *r1 {
            let a = resolver.resolve(r2);
            // X {op} a = value => X = value {invop} a
            value = op.xe(&value, &a);
        } else {
            let a = resolver.resolve(r1);
            // a {op} X = value =>
            // op == Plus or Times or Equals, X = value {invop} a
            // op == Minus or Div, X = a {op} value
            value = match op {
                Operation::Plus | Operation::Times | Operation::Equals => op.xe(&value, &a),
                Operation::Minus | Operation::Div => op.ex(&a, &value),
            };
        }
    }
    value
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
    assert_eq!(res,152);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,232974643455000);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,301);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3740214169961);
}
