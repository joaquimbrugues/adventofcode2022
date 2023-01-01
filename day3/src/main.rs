use std::{env,process,fs};

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (l,r) = line.split_at(line.len() / 2);
        for cl in l.chars() {
            if r.contains(cl) {
                if cl.is_lowercase() {
                    sum += cl as u32 - 'a' as u32 + 1;
                } else {
                    sum += cl as u32 - 'A' as u32 + 27;
                }
                break;
            }
        }
    }
    sum
}

fn run2(input: &str) -> u32 {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        for c in lines[i].chars() {
            if lines[i+1].contains(c) && lines[i+2].contains(c) {
                if c.is_lowercase() {
                    sum += c as u32 - 'a' as u32 + 1;
                } else {
                    sum += c as u32 - 'A' as u32 + 27;
                }
                break;
            }
        }
        i += 3;
    }
    sum
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
    assert_eq!(res, 157);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 8088);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 70);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2522);
}
