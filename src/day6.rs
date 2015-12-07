use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn parse_nums(nums: &str) -> Vec<usize> {
    nums.split(',').map(|num| {
        return num.trim().parse()
                .ok()
                .expect("Non-integer specified as a position")
    }).collect()
}

pub fn parse_instruction(instruction : String) -> (String, usize, usize, usize, usize) {
    let split_instructions : Vec<&str> = instruction.split_whitespace().collect();
    if split_instructions[0] == "toggle".to_string() {
        let start : Vec<usize> = parse_nums(split_instructions[1]);
        let end : Vec<usize> = parse_nums(split_instructions[3]);
        return (split_instructions[0].to_string(), start[0], start[1], end[0], end[1]);
    } else {
        let start : Vec<usize> = parse_nums(split_instructions[2]);
        let end : Vec<usize> = parse_nums(split_instructions[4]);
        return (split_instructions[1].to_string(), start[0], start[1], end[0], end[1]);
    }
}

pub fn lights_left_on() -> usize {
    let mut lights : [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    for line in BufReader::new(File::open("src/light_instructions.txt").ok().expect("Bad file_name")).lines() {
        let (instruction, start_x, start_y, end_x, end_y) = parse_instruction(line.ok().expect("Bad line"));
        for y in start_y..end_y + 1 {
            for x in start_x..end_x + 1 {
                lights[x][y] = 
                    if instruction == "on" {
                        true
                    } else if instruction == "off" {
                        false
                    } else {
                        !lights[x][y]
                    };
            }
        }
    }
    lights.into_iter().map(|row| row.into_iter().filter(|x| **x).count())
        .fold(0, |sum, curr| sum + curr)
}
/*
pub fn brightness() -> usize {
    let mut lights : [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    for line in BufReader::new(File::open("src/light_instructions.txt").ok().expect("Bad file_name")).lines() {
        let (instruction, start_x, start_y, end_x, end_y) = parse_instruction(line.ok().expect("Bad line"));
        for y in start_y..end_y + 1 {
            for x in start_x..end_x + 1 {
                lights[x][y] +=
                    if instruction == "on" {
                        1
                    } else if instruction == "off" {
                        -1
                    } else {
                        2
                    };
                if lights[x][y] < 0 {
                    lights[x][y] = 0;
                }
            }
        }
    }
    lights.into_iter().map(|row| row.into_iter().fold(0, |sum : u32, curr| sum + curr))
        .fold(0, |sum, curr| sum + curr)
}
*/
