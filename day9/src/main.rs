use std::{env,fs,process};
use std::collections::HashSet;

#[derive(Clone,Copy,Debug)]
enum Dir { U, D, L, R, }

fn plus(pos: (i32,i32), n: i32, dir: &Dir) -> (i32,i32) {
    match dir {
        Dir::U => (pos.0, pos.1 + n),
        Dir::D => (pos.0, pos.1 - n),
        Dir::R => (pos.0 + n, pos.1),
        Dir::L => (pos.0 - n, pos.1),
    }
}

fn distance(x: (i32,i32), y: (i32,i32)) -> i32 {
    let c0 = (x.0 - y.0).abs();
    let c1 = (x.1 - y.1).abs();
    if c0 > c1 {
        c0
    } else {
        c1
    }
}

fn run1(input: &str) -> usize {
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in input.lines() {
        // Parse input
        let words = line.split(' ').collect::<Vec<&str>>();
        let dir = match words[0] {
            "R" => Dir::R,
            "L" => Dir::L,
            "U" => Dir::U,
            "D" => Dir::D,
            s => panic!("Unexpected input {s}"),
        };
        let n = words[1].parse::<i32>().unwrap();
        for _ in 0..n {
            // Move the head
            head = plus(head, 1, &dir);
            // If the supremum distance is 1 or less, do nothing
            if distance(head, tail) > 1 {
                // Move tail to the right place with respect to head
                tail = snap(head,tail);
                //tail = match dir {
                    //Dir::R => (head.0 - 1, head.1),
                    //Dir::L => (head.0 + 1, head.1),
                    //Dir::U => (head.0, head.1 - 1),
                    //Dir::D => (head.0, head.1 + 1),
                //};
                visited.insert(tail);
            }
        }
    }
    visited.len()
}

// "Snap" pos1 to pos0, and return the desired new position for pos1
// Assume that distance(pos0,pos1) > 1
// Moreover, in one of the coordinates we have that |x.i - y.i| <= 1
// We must place pos1 right behind pos0 in the other coordinate
fn snap(pos0: (i32,i32), pos1: (i32,i32)) -> (i32,i32) {
    if (pos0.0 - pos1.0).abs() <= 1 {
        // We move vertically, and just take pos0.0
        if pos0.1 < pos1.1 {
            // pos0 is below, follow it from above
            (pos0.0, pos0.1 + 1)
        } else {
            // pos0 is above, follow it from below
            (pos0.0, pos0.1 - 1)
        }
    } else if (pos0.1 - pos1.1).abs() <= 1 {
        // We move horizontally, and just take pos0.1
        if pos0.0 < pos1.0 {
            // pos0 is to the left, follow it from the right
            (pos0.0 + 1, pos0.1)
        } else {
            // pos0 is to the right, follow it from the left
            (pos0.0 - 1, pos0.1)
        }        
    } else if distance(pos0,pos1) == 2 {
        ((pos0.0 + pos1.0) / 2, (pos0.1 + pos1.1) / 2)
    } else {
        panic!("Unexpected error: pos0: {pos0:?}; pos1: {pos1:?}");
    }
}

fn run2(input: &str) -> usize {
    let mut snake = vec![(0,0);10];
    let mut visited = HashSet::new();
    visited.insert((0,0));
    for line in input.lines() {
        // Parse input
        let words = line.split(' ').collect::<Vec<&str>>();
        let dir = match words[0] {
            "R" => Dir::R,
            "L" => Dir::L,
            "U" => Dir::U,
            "D" => Dir::D,
            s => panic!("Unexpected input {s}"),
        };
        let n = words[1].parse::<i32>().unwrap();
        for _ in 0..n {
            // Move the head
            snake[0] = plus(snake[0], 1, &dir);
            // Move the rest of the snake
            for i in 1..snake.len() {
                // If the supremum distance is 1 or less, do nothing
                if distance(snake[i-1], snake[i]) > 1 {
                    // Move tail to the right place with respect to the knot
                    snake[i] = snap(snake[i-1], snake[i]);
                }
            }
            visited.insert(*snake.last().unwrap());
        }
    }
    visited.len()
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
    assert_eq!(res,13);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,6030);
}

#[test]
fn example21() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,1);
}

#[test]
fn example22() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,36);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,2545);
}
