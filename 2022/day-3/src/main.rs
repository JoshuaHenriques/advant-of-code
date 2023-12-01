use std::{
    // cmp,
    fs::File,
    io::{prelude::*, BufReader}, 
};

fn main() {
    let file = File::open("priorities.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    let mut sum_priorities: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let (left, right) = line.split_at(line.len() / 2);
        println!("left: {}\nright: {}\noriginal: {}", left, right, line);
    }    
}
