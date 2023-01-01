use std::{env,fs,process};
use std::collections::HashSet;

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
    let mut blocks : HashSet<(u32,u32)> = HashSet::new();
    let mut max_y = 0;

    // Process input into blocks
    for line in input.lines() {
        let iter = line.split("->");
        let mut last : Option<(u32,u32)> = None;
        for string in iter {
            let mut pair = string.split(',');
            let x : u32 = pair.next().unwrap().trim().parse().unwrap();
            let y : u32 = pair.next().unwrap().trim().parse().unwrap();
            if y > max_y {
                max_y = y;
            }
            if let Some(b) = last {
                if x == b.0 {
                    if y < b.1 {
                        for i_y in y..=b.1 {
                            blocks.insert((x,i_y));
                        }
                    } else {
                        for i_y in b.1..=y {
                            blocks.insert((x,i_y));
                        }
                    }
                } else {
                    if x < b.0 {
                        for i_x in x..=b.0 {
                            blocks.insert((i_x,y));
                        }
                    } else {
                        for i_x in b.0..=x {
                            blocks.insert((i_x,y));
                        }
                    }
                }
            }
            last = Some((x,y));
        }
    }

    println!("{max_y}");

    // Loop through falling sand
    let mut sand_set = 0;
    //let mut abyss = false;
    while !blocks.contains(&(500,0)) {
        // We add a new grain of sand at (500,0)
        let mut sand = (500,0);
        let mut falling = true;
        while falling {
            // FIRST VERSION
            // Check if the grain of sand can be stopped at all
            //if sand.1 >= max_y {
                //abyss = true;
                //break;
            //}

            // Fall straight down
            let mut next_sand = (sand.0, sand.1 + 1);
            // SECOND VERSION
            // Chech if we reached the floor
            if next_sand.1 == max_y + 2 {
                blocks.insert(sand);
                sand_set += 1;
                break;
            } 
            if !blocks.contains(&next_sand) {
                sand = next_sand;
                continue;
            }
            // Fall diagonally down to the left
            next_sand = (sand.0 - 1, sand.1 + 1);
            if !blocks.contains(&next_sand) {
                sand = next_sand;
                continue;
            }
            // Fall diagonally down to the right
            next_sand = (sand.0 + 1, sand.1 + 1);
            if !blocks.contains(&next_sand) {
                sand = next_sand;
                continue;
            }
            // Sand grain is set at previous position
            blocks.insert(sand);
            sand_set += 1;
            falling = false;
        }
    }
    println!("{sand_set}");
}
