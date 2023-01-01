use std::{env,fs,process};

fn run1(input: &str) -> i64 {
    let lines = input.lines().map(|s| s.trim()).collect::<Vec<&str>>();
    let mut registries = Vec::with_capacity(lines.len());
    registries.push(1);
    for line in lines {
        let last = *registries.last().unwrap();
        if line == "noop" {
            registries.push(last);
        } else if let Some(string) = line.strip_prefix("addx ") {
            let num = string.parse::<i64>().unwrap();
            registries.push(last);
            registries.push(last + num);
        } else {
            panic!("Unexpected input");
        }
    }
    let mut i = 19;
    let mut sum = 0;
    while i < registries.len() {
        sum += registries[i] * ((i + 1) as i64);
        i += 40;
    }
    sum
}

fn run2(input: &str) {
    let lines = input.lines().map(|s| s.trim()).collect::<Vec<&str>>();
    let mut registries = Vec::with_capacity(lines.len());
    registries.push(1);
    for line in lines {
        let last = *registries.last().unwrap();
        if line == "noop" {
            registries.push(last);
        } else if let Some(string) = line.strip_prefix("addx ") {
            let num = string.parse::<i64>().unwrap();
            registries.push(last);
            registries.push(last + num);
        } else {
            panic!("Unexpected input");
        }
    }
    //println!("{registries:?}");
    println!("{}", registries.len());
    let mut row = String::from("");
    for i in 0..241 {
        let ii = (i % 40) as i64;
        if i > 0 && ii == 0 {
            println!("{row}");
            row = String::from("");
        }
        if ii >= registries[i] - 1 && ii <= registries[i] + 1 {
            row.push('#');
        } else {
            row.push('.');
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

    run2(&input);
    //let res = run1(&input);
    //println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,13140);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,14780);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
