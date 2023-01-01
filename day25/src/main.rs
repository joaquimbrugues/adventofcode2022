use std::{env,fs,process};

fn snafu_to_dec(string: &str) -> i64 {
    let mut int = 0;
    for c in string.chars() {
        int *= 5;
        int += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            e => panic!("Unexpected character {e}"),
        }
    }
    int
}

fn dec_to_snafu(int: i64) -> String {
    let mut tmp = int;
    let mut digits = Vec::new();
    loop {
        tmp += 2;
        let digit = match tmp % 5 {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => panic!("Unreachable"),
        };
        digits.push(digit);
        if tmp < 5 {
            break;
        }
        tmp /= 5;
    }
    let mut string = String::from("");
    digits.reverse();
    for c in digits {
        string.push(c);
    }
    string
}

fn run1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let int = snafu_to_dec(line);
        //println!("{line} : {int} : {}", dec_to_snafu(int));
        sum += int;
    }
    dec_to_snafu(sum)
}

//fn run2(input: &str) -> u32 {
    //0
//}

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

    let res = run1(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,"2=-1=0");
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,"2-1-110-=01-1-0-0==2");
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
