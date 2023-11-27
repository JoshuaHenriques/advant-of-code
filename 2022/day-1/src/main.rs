use std::{
    // cmp,
    fs::File,
    io::{prelude::*, BufReader}, collections::BinaryHeap,
};

fn main() {
    let file = File::open("calories.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    // let mut max: u32 = 0;
    let mut heap = BinaryHeap::new();
    let mut curr_sum: u32 = 0;
    for line in lines {
        if line == "" {
            // max = cmp::max(max, currSum); 
            heap.push(curr_sum);
            curr_sum = 0;
        } else {
            let calorie = line.parse::<u32>().unwrap();
            curr_sum += calorie;
        }
    }

    // println!("{:?}", max);
    let top_three = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("{:?}", top_three);
}