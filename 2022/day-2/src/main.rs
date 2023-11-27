use std::{
    // cmp,
    fs::File,
    io::{prelude::*, BufReader}, 
    collections::HashMap,
};

fn main() {
    let file = File::open("results.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    let mut shapes = HashMap::new();
    shapes.insert("X".to_string(), 1);
    shapes.insert("Y".to_string(), 2);
    shapes.insert("Z".to_string(), 3);

    let mut total_score: u32 = 0;
    // let mut curr_score: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let results: Vec<&str> = line.split(" ").collect();
        let opp = results[0];
        let me = results[1];
        println!("opp: {:?}, me: {:?}", opp, me);
        // match line
    }
}