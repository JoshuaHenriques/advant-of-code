use std::{
    fs::File,
    // collections::HashMap,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could not parse line!"))
        .collect();

    let ROWS = lines.len();
    let COLS = lines[1].len();

    // println!("strlen: {}, lineslen: {}", lines[1].len(), lines.len());
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; COLS];ROWS];


    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            matrix[i][j] = char;    
        }
    }

    let mut curr_num: Vec<i32> = vec![];
    let mut valid: bool = false;

    for row in 0..=ROWS - 1 {
        for col in 0..=COLS - 1 {
            if matrix[row][col].is_numeric() {
                curr_num.push(matrix[row][col] as i32);
                
                if !valid {
                    let dir: Vec<Vec<i32>> = vec![[0, -1].to_vec(), [0, 1].to_vec(), [1, 0].to_vec(), [-1, 0].to_vec(), [-1, -1].to_vec(), [1, 1].to_vec(), [-1, 1].to_vec(), [1, -1].to_vec()];
                    
                    for dir_arr in dir {
                        let rdr = row + dir_arr[0];
                        let cdr = col + dir_arr[1];

                        if rdr < ROWS && cdc < COLS && matrix[rdr][cdc] != '.' && !matrix[rdr][cdc].is_numeric() {
                            valid = true;
                            break;
                        }
                        println!("{:?}", dir_arr);
                    }
                }
            } else {
                valid = false;
                curr_num = vec![];
            }
            // println!("{:?}", matrix[row][col]);
        } 
    }
    // println!("{:?}", matrix);
}
