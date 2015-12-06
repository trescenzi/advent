extern crate advent;

use advent::day1;
use advent::day2;

fn main() {
    println!("Santa has to go to floor: {}", day1::get_floor_num());
    println!("Santa gets to the basement first on floor: {}", day1::first_basement_visit());
    println!("Present surface area: {}", day2::sqft_needed());
    println!("Present bow_length_required: {}", day2::ribbon_needed());
}
