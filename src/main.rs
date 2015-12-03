extern crate advent;

use advent::day1;

fn main() {
    println!("Santa has to go to floor: {}", day1::get_floor_num());
    println!("Santa gets to the basement first on floor: {}", day1::first_basement_visit());
}
