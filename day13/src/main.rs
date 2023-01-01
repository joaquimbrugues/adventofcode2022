use std::{env,fs,process};
use std::cmp::Ordering;

#[derive(PartialEq,Eq,Clone)]
enum Message {
    Num(u8),
    List(Vec<Message>),
}

impl Message {
    /*
     *Parse line. We have cases:
     *    1- digits - keep parsing until we hit a non-digit character, format the resulting string as a number, and return Num(number) with a bool depending on the next character.
     *    2- ',' - return Num(number) and true
     *    3- ']' - return Num(number) and false
     *    4- '[' - initiate a sequence of calls to read until one of the calls returns false, then collect all of the elements into a Vec and return List of it
     */
    fn read(chars: &mut dyn Iterator<Item = char>) -> (Option<Self>, bool) {
        use Message::*;
        if let Some(c) = chars.next() {
            match c {
                '[' => {
                    let mut v = vec![];
                    let mut cont = true;
                    while cont {
                        let (mess, b) = Self::read(chars);
                        if mess.is_some() {
                            v.push(mess.unwrap());
                        }
                        cont = b;
                    }
                    return (Some(List(v)), true);
                },
                '0'..='9' => {
                    let mut digits = String::from(c);
                    let mut cont = true;
                    while let Some(c) = chars.next() {
                        match c {
                            '0'..='9' => digits.push(c),
                            ',' => {
                                cont = true;
                                break;
                            },
                            ']' => {
                                cont = false;
                                break;
                            }
                            t => {
                                panic!("Unexpected input {t}");
                            },
                        }
                    }
                    let num = digits.parse::<u8>().unwrap();
                    return (Some(Num(num)), cont);
                },
                ']' => {
                    return (None, false);
                },
                ',' => {
                    return (None, true);
                },
                t => {
                    panic!("Unexpected input {t}");
                }
            };
        } else {
            panic!("Input ended unexpectedly!");
        }
    }

    fn parse(string: &str) -> Self {
        let (res, _) = Self::read(&mut string.chars());
        if let Some(m) = res {
            return m;
        } else {
            panic!("Whatever, just die");
        }
    }

    // For testing purposes
    fn to_string(&self) -> String {
        use Message::*;

        match self {
            Num(n) => n.to_string(),
            List(v) => {
                let mut s = String::from("List(");
                let mut i = 0;
                for m in v {
                    if i > 0 { s.push(','); }
                    s.push_str(&m.to_string());
                    i += 1;
                }
                s.push(')');
                s
            }
        }
    }
}

fn insert_ordered(vec: &mut Vec<Message>, el: Message) -> usize {
    let mut i = 0;
    while i < vec.len() && vec[i] < el {
        i += 1;
    }
    vec.insert(i,el);
    i + 1
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        use Message::*;

        match (self, other) {
            (Num(a), Num(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Num(a), List(b)) => vec![Num(*a)].cmp(b),
            (List(a), Num(b)) => a.cmp(&vec![Num(*b)]),
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run1(input: &str) -> u32 {
    let pairs = input.split("\n\n");
    let mut i = 1;
    let mut sum = 0;
    for pairstr in pairs {
        let pair: Vec<Message> = pairstr.trim().split('\n').map(|line| Message::parse(line)).collect();
        if pair[0] < pair[1] {
            sum += i;
        }
        i += 1;
    }
    sum
}

fn run2(input: &str) -> usize {
    let mut vec = vec![];
    for line in input.lines() {
        if line.len() > 0 {
            let line = line.trim();
            insert_ordered(&mut vec, Message::parse(line));
        }
    }
    let i1 = insert_ordered(&mut vec, Message::parse("[[2]]"));
    let i2 = insert_ordered(&mut vec, Message::parse("[[6]]"));
    i1 * i2
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
    assert_eq!(res, 13);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 5330);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 140);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 27648);
}
