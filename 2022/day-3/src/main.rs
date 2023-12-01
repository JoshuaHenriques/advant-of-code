use std::{
    // cmp,
    fs::File,
    collections::HashSet,
    io::{prelude::*, BufReader}, 
};

fn main() {
    let file = File::open("priorities.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    let mut sum_priorities: i32 = 0;
    let mut priorities = HashSet::<char>::new();
    for (_, line) in lines.iter().enumerate() {
        let (left, right) = line.split_at(line.len() / 2);
        // println!("left: {}\nright: {}\noriginal: {}", left, right, line);

        for char in left.chars() {
            priorities.insert(char);
        }
        
        for char in right.chars() {
            if priorities.contains(&char) {
                let a = 'a' as i32;
                let char_num = char as i32;
                println!("{}: {}", char_num, a); 
                sum_priorities += char_num - a;
                break;
            }
        }
    }    

    println!("{}", sum_priorities);
}
