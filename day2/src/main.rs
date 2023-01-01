use std::{env,process,fs};

#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {

   /*
    * Give value of given play:
    *    Rock -> 1
    *    Paper -> 2
    *    Scissors -> 3
    */
    fn value(&self) -> u32 {
        use RPS::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn readchar(c: char) -> Self {
        use RPS::*;

        match c.to_ascii_lowercase() {
            'a' | 'x' => Rock,
            'b' | 'y' => Paper,
            'c' | 'z' => Scissors,
            _ => {
                panic!("Unexpected character!");
            }
        }
    }
}

/*
 *Return result of competition, given by the value of 'you' play and the score:
 *    lose -> 0
 *    draw -> 3
 *    win -> 6
 */
fn rockpaperscissors(opponent: RPS, you: RPS) -> u32 {
    use RPS::*;
    let comp = match opponent {
        Rock => {
            match you {
                Rock => 3,
                Paper => 6,
                Scissors => 0,
            }
        },
        Paper => {
            match you {
                Rock => 0,
                Paper => 3,
                Scissors => 6,
            }
        },
        Scissors => {
            match you {
                Rock => 6,
                Paper => 0,
                Scissors => 3,
            }
        }
    };
    comp + you.value()
}

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let mut cont = line.chars();
        let elf = match cont.next() {
            Some(c) => RPS::readchar(c),
            None => {
                panic!("Malformed input!");
            }
        };
        cont.next();
        let you = match cont.next() {
            Some(c) => RPS::readchar(c),
            None => {
                panic!("Malformed input!");
            }
        };
        sum += rockpaperscissors(elf,you);
    }
    sum
}

fn run2(input: &str) -> u32 {
    use RPS::*;

    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let mut cont = line.chars();
        let elf = match cont.next() {
            Some(c) => RPS::readchar(c),
            None => {
                panic!("Malformed input!");
            }
        };
        cont.next();
        if let Some(c) = cont.next() {
            match c.to_ascii_lowercase() {
                'x' => {
                    // Losing
                    let s = match elf {
                        Rock => Scissors.value(),
                        Paper => Rock.value(),
                        Scissors => Paper.value(),
                    };
                    sum += s;
                },
                'y' => {
                    // Draw
                    sum += 3 + elf.value();
                },
                'z' => {
                    // Win
                    sum += 6;
                    let s = match elf {
                        Rock => Paper.value(),
                        Paper => Scissors.value(),
                        Scissors => Rock.value(),
                    };
                    sum += s;
                }
                _ => {
                    panic!("Unexpected input!");
                }
            }
        } else {
            panic!("Malformed input!");
        }
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

    let temp = fs::read_to_string(filepath).unwrap();
    let s = run2(&temp);
    println!("{s}");
}

#[test]
fn test_example1() {
    let temp = fs::read_to_string("test.txt").unwrap();
    let s = run1(&temp);
    assert_eq!(s, 15);
}

#[test]
fn test_input1() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let s = run1(&temp);
    assert_eq!(s, 14264);
}

#[test]
fn test_example2() {
    let temp = fs::read_to_string("test.txt").unwrap();
    let s = run2(&temp);
    assert_eq!(s, 12);
}
