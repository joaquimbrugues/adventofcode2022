use std::{env,fs,process};
use std::collections::{VecDeque,BinaryHeap};

enum Atom {
    Num(u64),
    X,
}

impl Atom {
    fn parse(string: &str) -> Self {
        if string == "old" {
            Atom::X
        } else {
            let num = string.parse().unwrap();
            Atom::Num(num)
        }
    }

    fn replace(&self, rep: u64) -> u64 {
        match self {
            Atom::Num(n) => *n,
            Atom::X => rep,
        }
    }
}

enum Operation {
    Sum(Atom, Atom),
    Product(Atom, Atom),
}

impl Operation {
    fn parse(string: &str) -> Self {
        let words: Vec<&str> = string.split(' ').collect();
        let atom1 = Atom::parse(words[0]);
        let atom2 = Atom::parse(words[2]);
        match words[1] {
            "+" => Operation::Sum(atom1,atom2),
            "*" => Operation::Product(atom1,atom2),
            s => panic!("Unexpected input {s}"),
        }
    }

    fn evaluate(&self, ev: u64, modulo: u32) -> u64 {
        match self {
            Operation::Sum(a1,a2) => (a1.replace(ev) + a2.replace(ev)) % (modulo as u64),
            Operation::Product(a1,a2) => (a1.replace(ev) * a2.replace(ev)) % (modulo as u64),
        }
    }
}

struct Monkey {
    id: usize,
    held_items: VecDeque<u64>,
    operation: Operation,
    test: u32,
    to_true: usize,
    to_false: usize,
    inspected: u64,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let s1 = lines.next().unwrap().strip_prefix("Monkey ").unwrap().strip_suffix(':').unwrap();
        let id = s1.parse().unwrap();
        let s2 = lines.next().unwrap().trim().strip_prefix("Starting items: ").unwrap();
        let mut held_items = VecDeque::new();
        for s in s2.split(", ") {
            held_items.push_back(s.parse().unwrap());
        }
        let s3 = lines.next().unwrap().trim().strip_prefix("Operation: new = ").unwrap();
        let operation = Operation::parse(s3);
        let s4 = lines.next().unwrap().trim().strip_prefix("Test: divisible by ").unwrap();
        let test = s4.parse().unwrap();
        let s5 = lines.next().unwrap().trim().strip_prefix("If true: throw to monkey ").unwrap();
        let to_true = s5.parse().unwrap();
        let s6 = lines.next().unwrap().trim().strip_prefix("If false: throw to monkey ").unwrap();
        let to_false = s6.parse().unwrap();
        Self { id, held_items, operation, test, to_true, to_false, inspected: 0, }
    }

    fn inspect(&mut self, divide: bool, modulo: u32) -> Option<(usize,u64)> {
        if let Some(mut worry) = self.held_items.pop_front() {
            worry = self.operation.evaluate(worry,modulo);
            self.inspected += 1;
            if divide {
                worry /= 3;
            }
            if worry % (self.test as u64) == 0 {
                Some((self.to_true, worry))
            } else {
                Some((self.to_false, worry))
            }
        } else {
            None
        }
    }
}

fn mcd(a: &u32, b: &u32) -> u32 {
    if *b == 1 {
        return *a;
    }
    mcd(b, &(a % b))
}

fn mcm(a: &u32, b: &u32) -> u32 {
    (a * b) / mcd(a,b)
}

fn run1(input: &str) -> u64 {
    let mut monkeys = Vec::new();
    let mut mult = 1;
    // Read input
    for string in input.split("\n\n") {
        monkeys.push(Monkey::parse(string));
        mult *= monkeys.last().unwrap().test;
    }
    // 20 rounds of monkeys inspecting elements
    let number_rounds = 20;
    for _ in 0..number_rounds {
        for i in 0..monkeys.len() {
            while let Some((id,worry)) = monkeys[i].inspect(true, mult) {
                monkeys[id].held_items.push_back(worry);
            }
        }
    }
    // Sort monkeys by how many items they inspected
    let mut priority = BinaryHeap::new();
    for monkey in &monkeys {
        priority.push(monkey.inspected);
    }
    // Multiply the two greater numbers
    priority.pop().unwrap() * priority.pop().unwrap()
}

fn run2(input: &str) -> u64 {
    let mut monkeys = Vec::new();
    let mut mult = 1;
    // Read input
    for string in input.split("\n\n") {
        monkeys.push(Monkey::parse(string));
        mult *= monkeys.last().unwrap().test;
    }
    // 10000 rounds of monkeys inspecting elements
    let number_rounds = 10000;
    for _ in 0..number_rounds {
        for i in 0..monkeys.len() {
            while let Some((id,worry)) = monkeys[i].inspect(false, mult) {
                monkeys[id].held_items.push_back(worry);
            }
        }
    }
    // Sort monkeys by how many items they inspected
    let mut priority = BinaryHeap::new();
    for monkey in &monkeys {
        priority.push(monkey.inspected);
    }
    // Multiply the two greater numbers
    priority.pop().unwrap() * priority.pop().unwrap()
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
    assert_eq!(res,10605);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,90882);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,2713310158);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,30893109657);
}
