use std::fs::File;
use std::io::Read;

fn follow_directions(directions: String) -> i32 {
    return directions.chars()
        .fold(0, |acc, curr| {
            match curr {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc
            }
        });
}

pub fn get_floor_num() -> i32 {
    let mut directions = String::new();
    match File::open("src/directions.txt") {
        Ok(mut f) => f.read_to_string(&mut directions).ok().expect("string failed"),
        Err(_) => return -42
    };
    return follow_directions(directions);
}
