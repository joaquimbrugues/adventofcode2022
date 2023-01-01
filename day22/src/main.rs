use std::{env,fs,process};

// Rules:
// 1- When we walk straight, if we hit a wall, just stop.
// 2- If we hit the end of the map, just wrap around.
// 3- R means turning clockwise, L means turning counterclockwise
// 4- Solution is 1000*row + 4*column + facing, where row and column start at 1, and facing is
// determined by > - 0; v - 1; < - 2; ^ - 3;

#[derive(Clone,Copy,Debug)]
enum Dir { East, South, West, North, }

impl Dir {
    fn enumerate() -> Vec<Self> {
        use Dir::*;
        vec![East, South, West, North]
    }

    fn to_num(&self) -> u8 {
        use Dir::*;
        match self {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }

    fn plus(&self, d: char) -> Self {
        let ind = self.to_num() as i8;
        let offset: i8 = match d {
            'L' => 3, // Since we are in Z/4Z, -1 = 3
            'R' => 1,
            c => {
                panic!("Unexpected char {c}");
            },
        };
        Self::enumerate()[((ind + offset) % 4) as usize]
    }
}

fn walk(pos: (usize,usize), dir: Dir, matrix: &Vec<Vec<char>>, layout: usize) -> Option<((usize, usize),Dir)> {
    match layout {
        1 => walk1(pos,dir,matrix),
        2 => walk2(pos,dir,matrix),
        3 => walk3(pos,dir,matrix),
        _ => {
            panic!("Unimplemented");
        },
    }
}

// Do one step in the given direction
// Remember: first coordinate denotes y-axis, second coordinate denotes x-axis
fn walk1(pos: (usize,usize), dir: Dir, matrix: &Vec<Vec<char>>) -> Option<((usize, usize),Dir)> {
    use Dir::*;
    match dir {
        East => {
            let mut nx = pos.1;
            loop {
                nx = (nx + 1) % matrix[pos.0].len();
                if matrix[pos.0][nx] != ' ' {
                    break;
                }
            }
            if matrix[pos.0][nx] == '#' {
                None
            } else {
                Some(((pos.0,nx),dir))
            }
        },
        West => {
            let mut nx = pos.1;
            let len = matrix[pos.0].len();
            loop {
                nx = (nx + len - 1) % len; // Since we are in Z/lenZ, -1 = len - 1
                if matrix[pos.0][nx] != ' ' {
                    break;
                }
            }
            if matrix[pos.0][nx] == '#' {
                None
            } else {
                Some(((pos.0,nx),dir))
            }
        },
        South => {
            let mut ny = pos.0;
            loop {
                ny = (ny + 1) % matrix.len();
                if matrix[ny].len() <= pos.1 {
                    ny = 0;
                }
                if matrix[ny][pos.1] != ' ' {
                    break;
                }
            }
            if matrix[ny][pos.1] == '#' {
                None
            } else {
                Some(((ny, pos.1),dir))
            }
        },
        North => {
            let mut ny = pos.0;
            let len = matrix.len();
            loop {
                ny = (ny + len - 1) % len; // Since we are in Z/lenZ, -1 = len - 1
                while matrix[ny].len() <= pos.1 {
                    ny -= 1;
                }
                if matrix[ny][pos.1] != ' ' {
                    break;
                }
            }
            if matrix[ny][pos.1] == '#' {
                None
            } else {
                Some(((ny, pos.1),dir))
            }
        },
    }
}

// Do one step in the given direction
// Remember: first coordinate denotes y-axis, second coordinate denotes x-axis
// Also, take into account the folding of the cube
fn walk2(pos: (usize,usize), dir: Dir, matrix: &Vec<Vec<char>>) -> Option<((usize, usize), Dir)> {
    use Dir::*;
    let mut nx = pos.1;
    let mut ny = pos.0;
    let facelen = matrix.len() / 3;
    let mut ndir = dir;
    match dir {
        East => {
            nx += 1;
            if nx >= matrix[pos.0].len() {
                // We got out of the map though the right side.
                // Classify new position and direction according to pos.0
                if pos.0 < facelen {
                    // Top square. We get back at the bottom right square with direction West
                    ny = matrix.len() - 1 - pos.0;
                    nx = matrix[ny].len() - 1;
                    ndir = West;
                } else if facelen <= pos.0 && pos.0 < 2 * facelen {
                    // Middle square. We get back at the bottom right square with direction South
                    ny = 2 * facelen;
                    nx = matrix[ny].len() + facelen - 1 - pos.0;
                    ndir = South;
                } else {
                    // Bottom right square. We get back at the top square with direction West
                    ny = matrix.len() - 1 - pos.0;
                    nx = matrix[ny].len() - 1;
                    ndir = West;
                }
            }
        },
        West => {
            if nx == 0 || matrix[pos.0][nx - 1] == ' ' {
                // We got out of the map though the left side.
                // Classify new position and direction according to pos.0
                if pos.0 < facelen {
                    // Top square. We get back at the second square of the second row with
                    // direction South
                    nx = pos.0 + facelen;
                    ny = facelen;
                    ndir = South;
                } else if facelen <= pos.0 && pos.0 < 2 * facelen {
                    // Medium square. We get back at the bottom right square with direction North
                    ny = matrix.len() - 1;
                    nx = matrix[ny].len() + facelen - 1 - pos.0;
                    ndir = North;
                } else {
                    // Lower square. We get back at the second square of the second row with
                    // direction North
                    ny = 2 * facelen - 1;
                    nx = matrix.len() + facelen - 1 -  pos.0;
                    ndir = North;
                }
            } else {
                nx -= 1;
            }
        },
        South => {
            ny += 1;
            if ny >= matrix.len() || matrix[ny][pos.1] == ' ' {
                // We got out of the map though the bottom side.
                // Classify new position and direction according to pos.1
                if pos.1 < facelen {
                    // Leftmost square. We get back at the first square in the bottom row in
                    // direction North
                    ny = matrix.len() - 1;
                    nx = (3 * facelen) - pos.1 - 1;
                    ndir = North;
                } else if pos.1 < 2 * facelen {
                    // Second square. We get back at the first square in the bottom row in
                    // direction East
                    nx = 2 * facelen;
                    ny = matrix.len() + facelen - pos.1 - 1;
                    ndir = East;
                } else if pos.1 < 3 * facelen {
                    // First square bottom row. We get back at the first square of the second row
                    // in direction North
                    ny = 2 * facelen - 1;
                    nx = 3 * facelen - pos.1 - 1;
                    ndir = North;
                } else {
                    // Second square bottom row. We get back at the first square of the second row
                    // in direction East
                    nx = 0;
                    ny = 5 * facelen - pos.1 - 1;
                    ndir = East;
                }
            }
        },
        North => {
            if ny == 0 || matrix[ny - 1][pos.1] == ' ' {
                // We got out of the map though the top side.
                // Classify new position and direction according to pos.1
                if pos.1 < facelen {
                    // First square second row. We get back at the top square in direction South
                    ny = 0;
                    nx = 3 * facelen - pos.1 - 1;
                    ndir = South;
                } else if pos.1 < 2 * facelen {
                    // Second square second row. We get back at the top square in direction East
                    nx = 2 * facelen;
                    ny = pos.1 - facelen;
                    ndir = East;
                } else if pos.1 < 3 * facelen {
                    // Top square. We get at first square second row in direction South
                    ny = facelen;
                    nx = 3 * facelen - pos.1 - 1;
                    ndir = South;
                } else {
                    // Bottom right square. We get back at the rightmost square in second row with
                    // direction West
                    nx = 3 * facelen - 1;
                    ny = 5 * facelen - pos.1 - 1;
                    ndir = West;
                }
            } else {
                ny -= 1;
            }
        },
    }
    if matrix[ny][nx] == '#' {
        None
    } else {
        Some(((ny,nx), ndir))
    }
}

// Do one step in the given direction
// Remember: first coordinate denotes y-axis, second coordinate denotes x-axis
// Also, take into account the folding of the cube
fn walk3(pos: (usize,usize), dir: Dir, matrix: &Vec<Vec<char>>) -> Option<((usize, usize), Dir)> {
    use Dir::*;
    let mut nx = pos.1;
    let mut ny = pos.0;
    let facelen = matrix.len() / 4;
    let mut ndir = dir;
    match dir {
        East => {
            nx += 1;
            if nx >= matrix[pos.0].len() {
                // We got out of the map though the right side.
                // Classify new position and direction according to pos.0
                if pos.0 < facelen {
                    // Top square. We get back at the second square on third row facing West
                    nx = 2 * facelen - 1;
                    ny = 3 * facelen - pos.0 - 1;
                    ndir = West;
                } else if pos.0 < 2 * facelen {
                    // Second row square. We get back at the top right square with direction North
                    ny = facelen - 1;
                    nx = facelen + pos.0;
                    ndir = North;
                } else if pos.0 < 3 * facelen {
                    // Third row square. We get back at the top right square with direction West
                    nx = 3 * facelen - 1;
                    ny = 3 * facelen - pos.0 - 1;
                    ndir = West;
                } else {
                    // Bottom square. We get back at the third row square with direction North
                    ny = 3 * facelen - 1;
                    nx = pos.0 - (2 * facelen);
                    ndir = North;
                }
            }
        },
        West => {
            if nx == 0 || matrix[pos.0][nx - 1] == ' ' {
                // We got out of the map though the left side.
                // Classify new position and direction according to pos.0
                if pos.0 < facelen {
                    // Top square. We get back at the first third row square with
                    // direction East
                    nx = 0;
                    ny = 3 * facelen - pos.0 - 1;
                    ndir = East;
                } else if pos.0 < 2 * facelen {
                    // Second row square. We get back at the first square in the third row with
                    // direction South
                    ny = 2 * facelen;
                    nx = pos.0 - facelen;
                    ndir = South;
                    
                } else if pos.0 < 3 * facelen {
                    // Third row square. We get back at the first row square with direction East
                    nx = facelen;
                    ny = 3 * facelen - pos.0 - 1;
                    ndir = East;
                } else {
                    // Bottom row square. We get back at the first square of the top row with
                    // direction South
                    ny = 0;
                    nx = pos.0 - 2 * facelen;
                    ndir = South;
                }
            } else {
                nx -= 1;
            }
        },
        South => {
            ny += 1;
            if ny >= matrix.len() || nx >= matrix[ny].len() || matrix[ny][pos.1] == ' ' {
                // We got out of the map though the bottom side.
                // Classify new position and direction according to pos.1
                if pos.1 < facelen {
                    // Bottom square. We get back at the second square in the top row in
                    // direction South
                    ny = 0;
                    nx = pos.1 + 2 * facelen;
                    ndir = South;
                } else if pos.1 < 2 * facelen {
                    // Third row square. We get back at the bottom row square with
                    // direction West
                    nx = facelen - 1;
                    ny = pos.1 + 2 * facelen;
                    ndir = West;
                } else {
                    // Second square on the top row. We get back at the second row square
                    // in direction West
                    nx = 2 * facelen - 1;
                    ny = pos.1 - facelen;
                    ndir = West;
                }
            }
        },
        North => {
            if ny == 0 || matrix[ny - 1][pos.1] == ' ' {
                // We got out of the map though the top side.
                // Classify new position and direction according to pos.1
                if pos.1 < facelen {
                    // First square on the third row. We get back at the second row square in direction East
                    nx = facelen;
                    ny = pos.1 + facelen;
                    ndir = East;
                } else if pos.1 < 2 * facelen {
                    // First square in top row. We get to the square in the bottom row with
                    // direction East
                    nx = 0;
                    ny = pos.1 + 2 * facelen;
                    ndir = East;
                } else {
                    // Top right square. We get back at the bottom row square with direction North
                    ny = 4 * facelen - 1;
                    nx = pos.1 - 2 * facelen;
                    ndir = North;
                }
            } else {
                ny -= 1;
            }
        },
    }
    if matrix[ny][nx] == '#' {
        None
    } else {
        Some(((ny,nx), ndir))
    }
}

fn printpos(pos: (usize, usize), dir: Dir, matrix: &Vec<Vec<char>>) {
    println!("{dir:?}");
    let mut y = 0;
    for row in matrix {
        let mut x = 0;
        let mut string = String::from("");
        for c in row {
            if pos == (y,x) {
                string.push('X');
            } else {
                string.push(*c);
            }
            x += 1;
        }
        println!("{string}");
        y += 1;
    }
    println!("");
}

fn run(input: &str, layout: usize) -> usize {
    use Dir::*;
    // IMPORTANT: First coordinate denotes y-axis, second coordinate denotes x-axis
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        matrix.push(line.chars().collect());
    }
    let instructions = lines.next().unwrap();
    let mut pos: (usize,usize) = (0,0);
    while matrix[0][pos.1] == ' ' {
        pos.1 += 1;
    }
    let mut dir = East;
    let mut string = String::from("");
    for c in instructions.chars() {
        if c.is_ascii_digit() {
            string.push(c);
        } else {
            //println!("{string} {dir:?}");
            let num = string.parse::<usize>().unwrap();
            let mut i = 0;
            while i < num {
                match walk(pos, dir, &matrix, layout) {
                    Some((npos,ndir)) => {
                        pos = npos;
                        dir = ndir;
                    },
                    None => break,
                }
                i += 1;
            }
            dir = dir.plus(c);
            //printpos(pos, dir, &matrix);
            string = String::from("");
            //println!("{pos:?}");
        }
    }
    if string.len() > 0 {
        let num = string.parse::<usize>().unwrap();
        let mut i = 0;
        while i < num {
            match walk(pos, dir, &matrix, layout) {
                Some((npos,ndir)) => {
                    pos = npos;
                    dir = ndir;
                },
                None => break,
            }
            i += 1;
        }
    }
    (1000 * (pos.0 + 1)) + (4 * (pos.1 + 1)) + (dir.to_num() as usize)
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

    let res = run(&input,3);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,1);
    assert_eq!(res,6032);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input,1);
    assert_eq!(res,50412);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input,2);
    assert_eq!(res,5031);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run(&input,3);
    //assert_eq!(res,42);
//}
