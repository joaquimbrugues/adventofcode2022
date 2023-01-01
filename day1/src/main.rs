use std::{env,fs,process,io::{self,prelude::*,BufReader}};

fn main() -> io::Result<()> {
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

    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    
    // First part
    //let mut most_calories = 0;
    //let mut current_calories = 0;
    //for line in reader.lines() {
        //let contents = line?;
        //if contents.len() == 0 {
            //if current_calories > most_calories {
                //most_calories = current_calories;
            //}
            //current_calories = 0;
        //} else {
            //current_calories += contents.parse::<u32>().unwrap();
        //}
    //}

    //println!("{most_calories}");

    // Second part
    let mut calories_list = Vec::new();
    let mut current_calories = 0;
    
    for line in reader.lines() {
        let contents = line?;
        if contents.len() == 0 {
            if calories_list.len() == 0 {
                calories_list.push(current_calories);
            } else {
                for i in 0..calories_list.len() {
                    if current_calories > calories_list[i] {
                        calories_list.insert(i,current_calories);
                        if calories_list.len() > 3 {
                            calories_list.pop();
                        }
                        break;
                    }
                }
            }
            current_calories = 0;
        } else {
            current_calories += contents.parse::<u32>().unwrap();
        }
    }

    for i in 0..calories_list.len() {
        if current_calories > calories_list[i] {
            calories_list.insert(i,current_calories);
            if calories_list.len() > 3 {
                calories_list.pop();
            }
            break;
        }
    }

    for c in &calories_list {
        println!("{c}");
    }
    println!("Total calories: {}", calories_list.into_iter().sum::<u32>());
    Ok(())
}
