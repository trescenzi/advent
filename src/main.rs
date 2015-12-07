extern crate advent;

use advent::day1;
use advent::day2;
use advent::day5;
use advent::day6;

fn main() {
    println!("Santa has to go to floor: {}", day1::get_floor_num());
    println!("Santa gets to the basement first on floor: {}", day1::first_basement_visit());
    println!("Present surface area: {}", day2::sqft_needed());
    println!("Present bow_length_required: {}", day2::ribbon_needed());
    println!("Number of nice texts: {}", day5::num_nice_texts());
    println!("Number of nice texts take two: {}", day5::num_nice_texts_take_two());
    println!("Number of lights left on {}", day6::lights_left_on());
    println!("Brightness of lights left on {}", day6::brightness());
}
