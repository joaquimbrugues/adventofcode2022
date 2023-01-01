use std::{env,fs,process};

// PART 2
// Find the beacon in the given limits
// It must be outside of the same interval on each row as computed before
// I start from the end because I have the feeling that it is around there
fn part2(max: i64, sensors: &Vec<(i64, i64, i64)>) -> u64 {
    let mut y = max;
    while y > 0 {
        y -= 1;
        let mut x = 0;
        while x < max {
            let mut outside = true;
            let (mut SX, mut SY, mut SD) = (0,0,0);
            for(sx, sy, d) in sensors {
                (SX,SY,SD) = (*sx,*sy,*d);
                let d = (x -SX).abs() + (y - SY).abs();
                //println!("({x},{y}), and Sensor: ({SX},{SX}).\nDistance: {d}, distance to Beacon is {SD}");
                outside &= d > SD;
                if !outside {
                    break;
                }
            }
            if outside {
                return (x as u64) * 4000000 + (y as u64);
            } else {
                println!("({x},{y})");
                x = SX + SD - (y - SY).abs();
                println!("({x},{y})");
            }
            x += 1;
        }
    }
    0
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
    let max;
    if let Some(s) = args.next() {
        max = s.parse::<i64>().unwrap();
    } else {
        max = 20;
    }

    let input = fs::read_to_string(filepath).unwrap();
    let mut sensors = vec![];
    //let mut intervals = HashSet::new();
    for line in input.lines() {
        let mut iter = line.split(|c| c == ',' || c == ':' || c == '=');
        iter.next();    //Skip first text
        let sensor_x : i64 = iter.next().unwrap().parse().unwrap();
        iter.next();    //Skip more text
        let sensor_y : i64 = iter.next().unwrap().parse().unwrap();
        iter.next();    //Skip more text
        let beacon_x : i64 = iter.next().unwrap().parse().unwrap();
        iter.next();    //Skip more text
        let beacon_y : i64 = iter.next().unwrap().parse().unwrap();
        let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        sensors.push((sensor_x, sensor_y, dist));

        // PART 1
        // Count points in the line y = given value that cannot possibly contain a beacon
        // This means, points in the line y within the interval (sensor_x - (dist - abs(sensor_y-y)), sensor_x + (dist - abs(sensor_y -y)))
        //let disty = (sensor_y - y).abs();
        //if dist >= disty {
            //intervals.extend((sensor_x - (dist - disty))..(sensor_x + (dist - disty)));
        //}
    }
    //println!("{}", intervals.len());
    println!("{}", part2(max, &sensors));
}
