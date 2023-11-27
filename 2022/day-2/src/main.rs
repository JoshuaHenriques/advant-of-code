use std::{
    // cmp,
    fs::File,
    io::{prelude::*, BufReader}, 
};

fn main() {
    let file = File::open("results.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    let mut total_score: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let results: Vec<&str> = line.split(" ").collect();
        let opp = results[0];
        let me = results[1];
        // println!("opp: {:?}, me: {:?}", opp, me);
        
        match opp {
            "A" => {
                match me {
                    "X" => total_score += 3,
                    "Y" => total_score += 1 + 3,
                    "Z" => total_score += 2 + 6,
                    // "X" => total_score += 1 + 3,
                    // "Y" => total_score += 2 + 6,
                    // "Z" => total_score += 3,
                    _ => (),
                }
            },
            "B" => {
                match me {
                    "X" => total_score += 1,
                    "Y" => total_score += 2 + 3,
                    "Z" => total_score += 3 + 6,
                    // "X" => total_score += 1,
                    // "Y" => total_score += 2 + 3,
                    // "Z" => total_score += 3 + 6,
                    _ => (),
                }
            },
            "C" => {
                match me {
                    "X" => total_score += 2,
                    "Y" => total_score += 3 + 3,
                    "Z" => total_score += 1 + 6,
                    // "X" => total_score += 1 + 6,
                    // "Y" => total_score += 2,
                    // "Z" => total_score += 3 + 3,
                    _ => (),
                }
            },
            _ => (),
        }
    }

    println!("{:?}", total_score);
}