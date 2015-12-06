use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn is_vowel(c : char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

pub fn is_naughty_combination(curr : char, prev : char) -> bool {
    if prev == 'a' && curr == 'b' {
        true
    } else if prev == 'c' && curr == 'd' {
        true
    } else if prev == 'p' && curr == 'q' {
        true
    } else if prev == 'x' && curr == 'y' {
        true
    } else {
        false
    }
}

pub fn num_nice_texts() -> usize {
    let mut nice_texts : usize = 0;
    'a: for line in BufReader::new(File::open("src/texts.txt").ok().expect("Bad file_name")).lines() {

        let line_str : String = line.ok().expect("Bad line provided in file");
        let mut num_vowels : usize = 0;
        let mut has_double : bool = false;
        let mut curr : char = ' ';
        let mut prev : char = ' ';

        for c in line_str.chars() {
            curr = c;

            if is_naughty_combination(curr, prev) {
                continue 'a;
            }

            if is_vowel(curr) {
                num_vowels += 1;
            }

            if !has_double && curr == prev {
                has_double = true;
            }
            prev = curr;
        }

        if has_double && num_vowels >= 3 {
            nice_texts += 1;
        }
    }
    nice_texts
}

fn all_equal(a : char, b : char, c : char) -> bool {
    return a == b && b == c && c == a;
}

pub fn num_nice_texts_take_two() -> usize {
    let mut nice_texts : usize = 0;
    let mut sep_doubles : usize = 0;
    let mut sep_pairs : usize = 0;
    for line in BufReader::new(File::open("src/texts.txt").ok().expect("Bad file_name")).lines() {

        let line_str : String = line.ok().expect("Bad line provided in file");
        let mut pairs : Vec<(char, char)> = Vec::new();
        let mut has_seperated_double = false;
        let mut has_double_pair = false;
        let mut curr : char;
        let mut prev : char = ' ';
        let mut prev_prev : char = ' ';

        for c in line_str.chars() {
            curr = c;

            if !has_seperated_double && curr == prev_prev {
                has_seperated_double = true;
                sep_doubles += 1;
            }

            if !has_double_pair && !all_equal(curr, prev, prev_prev) && pairs.contains(&(prev, curr)) {
                println!("double pair: str: {}, pair {:?}", line_str, (prev, curr));
                sep_pairs += 1;
                has_double_pair = true;
            }

            if has_double_pair && has_seperated_double {
                break;
            }

            if !has_double_pair {
                pairs.push((prev, curr));
            }
            
            prev_prev = prev;
            prev = curr;
        }

        if has_double_pair && has_seperated_double {
            nice_texts += 1;
        }
    }
    println!("seperated doubles: {} seperated pairs: {}", sep_doubles, sep_pairs);
    nice_texts
}
