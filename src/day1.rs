use std::fs::File;
use std::io::Read;
use std::iter::Scan;
use std::str::Chars;

fn interpret_direction(curr_floor: &mut i32, direction: char) -> Option<i32> {
    *curr_floor += match direction {
        '(' =>  1,
        ')' => -1,
        _ => 0,
    };

    return Some(*curr_floor);
}

type SDirections<'a> = Scan<Chars<'a>, i32, fn(&mut i32, char) -> Option<i32>>;

fn scan_directions<'a>(directions: &'a String) -> SDirections<'a> {
    return directions.chars().scan(0, interpret_direction);
}

fn read_directions_file(file_name: &'static str) -> String {
    let mut directions = String::new();
    match File::open(file_name) {
        Ok(mut f) => f.read_to_string(&mut directions).ok().expect("file read failed"),
        Err(e) => {
            println!("{}", e);
            return "File read failed".to_string();
        }
    };
    directions
}

pub fn get_floor_num() -> i32 {
    scan_directions(&read_directions_file("src/directions.txt")).last().unwrap()
}

pub fn first_basement_visit() -> u32 {
    let position: usize = scan_directions(&read_directions_file("src/directions.txt"))
        .position(|curr_floor| curr_floor == -1)
        .unwrap();
    position as u32
}
