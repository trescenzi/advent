use std::fs::File;
use std::io::Read;

fn scan_directions(directions: String) {
    return directions.chars()
        .scan(0, |acc, curr| {
            *acc += match curr {
                '(' =>  1,
                ')' => -1,
                _ => 0,
            };

            return Some(*acc);
        });
}

pub fn get_floor_num() -> i32 {
    let mut directions = String::new();
    match File::open("src/directions.txt") {
        Ok(mut f) => f.read_to_string(&mut directions).ok().expect("string failed"),
        Err(_) => return -42
    };
    return scan_directions(directions).last().unwrap();
}
