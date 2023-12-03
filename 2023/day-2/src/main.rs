use std::{
    fs::File,
    collections::HashMap,
    io::{prelude::*, BufReader}, 
};

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    for (_i, line) in lines.iter().enumerate() {

    }
}
