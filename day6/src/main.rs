use std::{env,fs,process};
use std::collections::HashSet;

fn run(input: &str, len: u16) -> u16 {
    let mut i = 0;
    let mut v = Vec::new();
    for c in input.chars() {
        v.push(c);
        i+= 1;
        if i >= len {
            let set: HashSet<char> = HashSet::from_iter(v[((i-len) as usize)..].into_iter().map(|c| *c));
            if set.len() == len.into() {
                return i;
            }
        }
    }
    0
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

    let res = run(&input,14);
    println!("{res}");
}

#[test]
fn example11() {
    let res = run("mjqjpqmgbljsphdztnvjfqwrcgsmlb",4);
    assert_eq!(res,7);
}

#[test]
fn example12() {
    let res = run("bvwbjplbgvbhsrlpgdmjqwftvncz",4);
    assert_eq!(res,5);
}

#[test]
fn example13() {
    let res = run("nppdvjthqldpwncqszvftbrmjlhg",4);
    assert_eq!(res,6);
}

#[test]
fn example14() {
    let res = run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4);
    assert_eq!(res,10);
}

#[test]
fn example15() {
    let res = run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4);
    assert_eq!(res,11);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,4);
    assert_eq!(res,1757);
}

#[test]
fn example21() {
    let res = run("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14);
    assert_eq!(res,19);
}

#[test]
fn example22() {
    let res = run("bvwbjplbgvbhsrlpgdmjqwftvncz",14);
    assert_eq!(res,23);
}

#[test]
fn example23() {
    let res = run("nppdvjthqldpwncqszvftbrmjlhg",14);
    assert_eq!(res,23);
}

#[test]
fn example24() {
    let res = run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14);
    assert_eq!(res,29);
}

#[test]
fn example25() {
    let res = run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14);
    assert_eq!(res,26);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,14);
    assert_eq!(res,2950);
}
