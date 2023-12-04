use std::{
    fs::File,
    // collections::HashMap,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|line| line.expect("Could nOt parse line!"))
        .collect();

    let ROWS: usize = lines.len();
    let COLS: usize = lines[1].len();

    // println!("strlen: {}, lineslen: {}", lines[1].len(), lines.len());
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; COLS];ROWS];


    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            matrix[i][j] = char;    
        }
    }

    let mut curr_num: Vec<isize> = vec![];
    let mut valid: bool = false;

    for row in 0..=ROWS - 1 {
        for col in 0..=COLS - 1 {
            if matrix[row as usize][col as usize].is_numeric() {
                curr_num.push(matrix[row as usize][col as usize] as isize);
                
                if !valid {
                    let dir: Vec<Vec<isize>> = vec![[0, -1].to_vec(), [0, 1].to_vec(), [1, 0].to_vec(), [-1, 0].to_vec(), [-1, -1].to_vec(), [1, 1].to_vec(), [-1, 1].to_vec(), [1, -1].to_vec()];
                    
                    for dir_arr in dir {
                        let rdr: isize = row + dir_arr[0];
                        let cdc: isize = col + dir_arr[1];

                        if rdr >= 0 && cdc >= 0 && rdr < ROWS && cdc < COLS && matrix[rdr as usize][cdc as usize] != '.' && !matrix[rdr as usize][cdc as usize].is_numeric() {
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
