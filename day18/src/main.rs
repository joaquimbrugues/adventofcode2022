use std::{env,fs,process};
use std::collections::HashSet;

fn minabs(a: u32, b: u32) -> u32 {
    match a > b {
        true => a - b,
        false => b - a,
    }
}

fn dist1(a: (u32, u32, u32), b: (u32, u32, u32)) -> u32 {
    minabs(a.0, b.0) + minabs(a.1, b.1) + minabs(a.2, b.2)
}

fn neighbours(coords: (u32, u32, u32)) -> Vec<(u32, u32, u32)> {
    let mut neighs = Vec::new();
    
    if coords.0 > 0 {
        neighs.push((coords.0 - 1, coords.1, coords.2));
    }
    neighs.push((coords.0 + 1, coords.1, coords.2));

    if coords.1 > 0 {
        neighs.push((coords.0, coords.1 - 1, coords.2));
    }
    neighs.push((coords.0, coords.1 + 1, coords.2));

    if coords.2 > 0 {
        neighs.push((coords.0, coords.1, coords.2 - 1));
    }
    neighs.push((coords.0, coords.1, coords.2 + 1));

    neighs
}

fn surf_bubbles(lava: &HashSet<(u32,u32,u32)>, max: (u32,u32,u32)) -> usize {
    let mut bubble_nodes = HashSet::new();
    let mut surf = 0;

    for x in 0..max.0 {
        for y in 0..max.1 {
            for z in 0..max.2 {
                let coords = (x,y,z);
                if !lava.contains(&coords) && !bubble_nodes.contains(&coords) {
                    // First, attempt to find a bubble
                    if let Some(bub) = find_bubble(lava, max, coords) {
                        for b in bub {
                            let neighs = neighbours(b);
                            for n in neighs {
                                if lava.contains(&n) {
                                    surf += 1;
                                }
                            }
                            bubble_nodes.insert(b);
                        }
                    }
                }
            }
        }
    }

    surf
}

fn find_bubble(lava: &HashSet<(u32,u32,u32)>, max: (u32,u32,u32), coords: (u32,u32,u32)) -> Option<HashSet<(u32,u32,u32)>> {
    if lava.contains(&coords) {
        return None;
    }
    let mut stack = Vec::new();
    stack.push(coords);
    let mut bubble = HashSet::new();
    while let Some(c) = stack.pop() {
        if !bubble.contains(&c) && !lava.contains(&c) {
            // Check if we reached open air
            if c.0 == 0 || c.1 == 0 || c.2 == 0 || c.0 == max.0 || c.1 == max.1 || c.2 == max.2 {
                return None;
            }

            // Depth-first search
            for n in neighbours(c) {
                stack.push(n);
            }
            bubble.insert(c);
        }
    }
    Some(bubble)
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

    let mut nodes = HashSet::new();
    let mut edges = 0;
    let mut max = (0,0,0);

    let input = fs::read_to_string(filepath).unwrap();
    for line in input.lines() {
        let mut split = line.split(',');
        let mut coords = (0,0,0);

        // 0
        if let Some(s) = split.next() {
            let c = s.parse::<u32>().unwrap();
            if c > max.0 {
                max.0 = c;
            }
            coords.0 = c;
        } else {
            eprintln!("Input ended unexpectedly");
            process::exit(1);
        }

        // 1
        if let Some(s) = split.next() {
            let c = s.parse::<u32>().unwrap();
            if c > max.1 {
                max.1 = c;
            }
            coords.1 = c;
        } else {
            eprintln!("Input ended unexpectedly");
            process::exit(1);
        }

        // 2
        if let Some(s) = split.next() {
            let c = s.parse::<u32>().unwrap();
            if c > max.2 {
                max.2 = c;
            }
            coords.2 = c;
        } else {
            eprintln!("Input ended unexpectedly");
            process::exit(1);
        }

        // Count edges
        for n in &nodes {
            if dist1(*n,coords) == 1 {
                edges += 1;
            }
        }

        nodes.insert(coords);
    }

    let num_faces = 6 * nodes.len() - 2 * edges - surf_bubbles(&nodes, max);
    //let num_faces = 6 * nodes.len() - 2 * edges;
    println!("Num faces: {num_faces}");
}
