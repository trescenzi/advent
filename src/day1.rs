use std::fs::File;
use std::io::Read;

fn interpret_direction(curr_floor: &mut i32, direction: char) -> Option<i32> {
    *curr_floor += match direction {
        '(' =>  1,
        ')' => -1,
        _ => 0,
    };

    return Some(*curr_floor);
}

fn scan_directions(directions: String) -> Box<Iterator<Item=i32>> {
    return Box::new(directions.chars()
        .scan(0, interpret_direction));
}

pub fn get_floor_num() -> i32 {
    let mut directions = String::new();
    match File::open("src/directions.txt") {
        Ok(mut f) => f.read_to_string(&mut directions).ok().expect("string failed"),
        Err(_) => return -42
    };
    return scan_directions(directions).last().unwrap();
}
