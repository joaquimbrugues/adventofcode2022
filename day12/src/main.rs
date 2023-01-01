use std::{env,fs,process};
//use std::collections::HashMap;
use std::collections::{HashSet,VecDeque};

fn to_value(c: char) -> u32 {
    match c {
        'S' => 'a' as u32,
        'E' => ('z' as u32) + 1,
        _ => c as u32,
    }
}

fn neighbours(array: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize,usize)> {
    let mut neighs = Vec::new();
    if x > 0 {
        if to_value(array[x][y]) + 1 >= to_value(array[x-1][y]) {
            neighs.push((x-1,y));
        }
    }

    if x < array.len() - 1 {
        if to_value(array[x][y]) + 1 >= to_value(array[x+1][y]) {
            neighs.push((x+1,y));
        }
    }

    if y > 0 {
        if to_value(array[x][y]) + 1 >= to_value(array[x][y-1]) {
            neighs.push((x,y-1));
        }
    }

    if y < array[x].len()-1 {
        if to_value(array[x][y]) + 1 >= to_value(array[x][y+1]) {
            neighs.push((x,y+1));
        }
    }
    neighs
}

//fn depth_first_search(x0: usize, y0: usize, array: &Vec<Vec<char>>) -> Option<u32> {
    //let mut stack = Vec::new();
    //// Stack contains x, y and depth
    //stack.push((x0,y0,0));
    //let mut visited = HashSet::new();
    //while let Some(xyd) = stack.pop(){
        //println!("In node ({},{}) with depth {}", xyd.0, xyd.1, xyd.2);
        ////Check if discovered
        //if !visited.contains(&(xyd.0,xyd.1)) {
            ////println!("Node ({},{}) not visited", xyd.0, xyd.1);
            ////Check if we are done!
            //if array[xyd.0][xyd.1] == 'E' {
                //return Some(xyd.2);
            //}

            ////Mark as discovered
            //visited.insert((xyd.0,xyd.1));

            ////Add neighbours to be visited
            //for n in neighbours(array,xyd.0,xyd.1) {
                ////println!("I'm a friend!");
                //stack.push((n.0,n.1,xyd.2+1));
            //}
        //}
    //}
    //None
//}

fn breadth_first_search(x0: usize, y0: usize, array: &Vec<Vec<char>>) -> Option<u32> {
    let mut queue = VecDeque::new();
    // Queue contains x, y and depth
    queue.push_back((x0,y0,0));
    let mut visited = HashSet::new();
    visited.insert((x0,y0));
    while let Some(xyd) = queue.pop_front(){
        println!("In node ({},{}) with depth {}", xyd.0, xyd.1, xyd.2);

        //Check if we are done!
        if array[xyd.0][xyd.1] == 'E' {
            return Some(xyd.2);
        }

        //Add neighbours to be visited
        for n in neighbours(array,xyd.0,xyd.1) {
            if !visited.contains(&(n.0,n.1)) {
                visited.insert((n.0,n.1));
                queue.push_back((n.0,n.1,xyd.2 + 1));
            }
        }
    }
    None
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
    let mut array : Vec<Vec<char>> = Vec::new();
    let mut n = 0;
    let mut start_position = (0,0);
    for line in input.lines() {
        let mut m = 0;
        let mut row = Vec::new();
        for c in line.chars() {
            if c=='S' {
                start_position = (n,m);
            }
            row.push(c);
            m += 1;
        }
        array.push(row);
        n += 1;
    }


    // First part
    //let result = breadth_first_search(start_position.0,start_position.1,&array);
    //match result {
        //Some(res) => {
            //println!("{res}");
        //}
        //None => {
            //println!("Not found");
        //}
    //}

    // Second part
    let mut best_result = 1000000;
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            if array[i][j] == 'S' || array[i][j] == 'a' {
                let result = breadth_first_search(i,j,&array);
                match result {
                    Some(res) => {
                        if res < best_result {
                            best_result = res;
                        }
                    },
                    None => {
                        println!("Not found!");
                    }
                }
            }
        }
    }
    println!("Best path: {}", best_result);
}
