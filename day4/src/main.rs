use std::{env,fs,process};

fn contains(a: (u8,u8), b: (u8,u8)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

fn read_line(line: &str) -> ((u8,u8),(u8,u8)) {
    let v: Vec<u8> = line.split(&['-', ',']).map(|s| s.parse::<u8>().unwrap()).collect();
    ((v[0], v[1]), (v[2], v[3]))
}

fn run1(input: &str) -> u16 {
    let mut num = 0;
    for line in input.lines() {
        let (a,b) = read_line(line.trim());
        if contains(a,b) || contains(b,a) {
            num += 1;
        }
    }
    num
}

fn run2(input: &str) -> u16 {
    let mut num = 0;
    for line in input.lines() {
        let (a,b) = read_line(line.trim());
        if a.0 <= b.1 && b.0 <= a.1 {
            num += 1;
        }
    }
    num
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
    assert_eq!(res,2);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,424);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,4);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,804);
}
