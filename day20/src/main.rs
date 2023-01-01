use std::{env,fs,process};

const DECRYPTION_KEY: i64 = 811589153;

fn run1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    let mut vec = Vec::with_capacity(len);
    let mut i = 0;
    for line in lines {
        let num = line.trim().parse::<i32>().unwrap();
        vec.push((i,num));
        i += 1;
    }
    for i in 0..len {
        let mut j = 0;
        while j < len && vec[j].0 != i {
            j += 1;
        }
        // We must move the item x to the position j + x.1 (mod len)
        let x = vec.remove(j);
        let index = ((j as i32) + x.1).rem_euclid((len - 1) as i32);
        if index == 0 {
            vec.push(x);
        } else {
            vec.insert(index as usize, x);
        }
    }
    let mut c0 = 0;
    for (i,(_,v)) in vec.iter().enumerate() {
        if *v == 0 {
            c0 = i;
            break;
        }
    }
    vec[(c0 + 1000) % len].1 + vec[(c0 + 2000) % len].1 + vec[(c0 + 3000) % len].1
}

fn run2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    let mut vec = Vec::with_capacity(len);
    let mut i = 0;
    for line in lines {
        let num = line.trim().parse::<i64>().unwrap() * DECRYPTION_KEY;
        vec.push((i,num));
        i += 1;
    }
    for _ in 0..10 {
        for i in 0..len {
            let mut j = 0;
            while j < len && vec[j].0 != i {
                j += 1;
            }
            // We must move the item x to the position j + x.1 (mod len)
            let x = vec.remove(j);
            let index = ((j as i64) + x.1).rem_euclid((len - 1) as i64);
            if index == 0 {
                vec.push(x);
            } else {
                vec.insert(index as usize, x);
            }
        }
    }
    let mut c0 = 0;
    for (i,(_,v)) in vec.iter().enumerate() {
        if *v == 0 {
            c0 = i;
            break;
        }
    }
    vec[(c0 + 1000) % len].1 + vec[(c0 + 2000) % len].1 + vec[(c0 + 3000) % len].1
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
    assert_eq!(res,3);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,988);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1623178306);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,7768531372516);
}
