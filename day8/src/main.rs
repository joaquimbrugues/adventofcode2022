use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> usize {
    let mut heights = Vec::new();
    // Read input
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        heights.push(row);
    }
    // Get all visible trees
    let mut visible = HashSet::with_capacity(heights.len() * heights[0].len());
    //From left and right
    for i in 0..heights.len() {
        let mut j = 1;
        visible.insert((i,0));
        let mut maxh = heights[i][0];
        while j < heights[i].len() {
            if maxh < heights[i][j] {
                visible.insert((i,j));
                maxh = heights[i][j];
            }
            j += 1;
        }
        j = heights[i].len() - 1;
        maxh = heights[i][j];
        visible.insert((i, j));
        loop {
            j -= 1;
            if maxh < heights[i][j] {
                visible.insert((i,j));
                maxh = heights[i][j];
            }
            if j == 0 {
                break;
            }
        }
    }
    // From up and down
    for j in 0..heights[0].len() {
        let mut i = 1;
        visible.insert((0,j));
        let mut maxh = heights[0][j];
        while i < heights.len() {
            if maxh < heights[i][j] {
                visible.insert((i,j));
                maxh = heights[i][j];
            }
            i += 1;
        }
        i = heights.len() - 1;
        visible.insert((i, j));
        maxh = heights[i][j];
        loop {
            i -= 1;
            if maxh < heights[i][j] {
                visible.insert((i,j));
                maxh = heights[i][j];
            }
            if i == 0 {
                break;
            }
        }
    }
    visible.len()
}

fn run2(input: &str) -> u32 {
    let mut heights = Vec::new();
    // Read input
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        heights.push(row);
    }
    let mut max = 0;
    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            let h = heights[i][j];
            let mut score = 1;
            // Count strictly lower trees to the left
            let mut sh = 0;
            let mut nj = j;
            while nj > 0 {
                nj -= 1;
                sh += 1;
                if heights[i][nj] >= h {
                    break;
                }
            }
            if sh > 0 {
                score *= sh;
            }
            // Count strictly lower trees to the right
            sh = 0;
            nj = j;
            while nj < heights[i].len() - 1 {
                nj += 1;
                sh += 1;
                if heights[i][nj] >= h {
                    break;
                }
            }
            if sh > 0 {
                score *= sh;
            }

            // Count strictly lower trees upward
            let mut sv = 0;
            let mut ni = i;
            while ni > 0 {
                ni -= 1;
                sv += 1;
                if heights[ni][j] >= h {
                    break;
                }
            }
            if sv > 0 {
                score *= sv;
            }
            // Count strictly lower trees downward
            sv = 0;
            ni = i;
            while ni < heights.len() - 1 {
                ni += 1;
                sv += 1;
                if heights[ni][j] >= h {
                    break;
                }
            }
            if sv > 0 {
                score *= sv;
            }

            println!("({i},{j}): {score}");
            if score > max {
                max = score;
            }
        }
    }
    max
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
    assert_eq!(res,21);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1835);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,16);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,263670);
}
