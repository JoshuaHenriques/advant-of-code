use substring::Substring;
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

    let mut map_nums = HashMap::new();
    map_nums.insert('o', vec!["one"]);
    map_nums.insert('t', vec!["two", "three"]);
    map_nums.insert('f', vec!["four", "five"]);
    map_nums.insert('s', vec!["six", "seven"]);
    map_nums.insert('e', vec!["eight"]);
    map_nums.insert('n', vec!["nine"]);

    let mut map_nums_values = HashMap::new();
    map_nums_values.insert("one", '1');
    map_nums_values.insert("two", '2');
    map_nums_values.insert("three", '3');
    map_nums_values.insert("four", '4');
    map_nums_values.insert("five", '5');
    map_nums_values.insert("six", '6');
    map_nums_values.insert("seven", '7');
    map_nums_values.insert("eight", '8');
    map_nums_values.insert("nine", '9');

    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; 
    let mut last = ' ';
    let mut first = ' ';
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if first == ' ' {
                    first = char;
                } else {
                    last = char;
                }
            } else if map_nums.contains_key(&char) {
                let arr = map_nums.get(&char);
                for (_, num) in arr.iter().enumerate() {
                    let substr = line.substring(j, j + num.len() - 1);
                    println!("substr: {:?}, num: {:?}, num.len(): {}", substr, num, num.len());
                    // if num == substr {
                    //     if first == ' ' {
                    //         first = *map_nums_values.get(num).unwrap();
                    //     } else {
                    //         last = *map_nums_values.get(num).unwrap();
                    //     }
                    // }
                }
            }
        }
        
        if first == ' ' {
            first = '0';
            last = '0';
        } else if last == ' ' {
            last = first;
        }

        let num = first.to_string() + &last.to_string();
        total += num.parse::<i32>().unwrap();

        println!("line: {}\nfirst: {}, last: {}, num: {}, total: {}",line, first, last, num, total);
        first = ' ';
        last = ' ';
    }

    println!("{}", total);
}
