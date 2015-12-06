use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::IntoIter;

fn get_present_dimmensions(present : &String) -> Option<Vec<usize>> {
    let dimmensions : Vec<usize> = present.split('x').map(|dimmension| {
        return dimmension.trim().parse()
                .ok()
                .expect("Non-integer specified as a dimmension")
    }).collect();
    assert_eq!(dimmensions.len(), 3);
    if dimmensions.len() == 3 {
        Some(dimmensions)
    } else {
        None
    }

}

fn wrapping_paper_required(present : &String) -> usize {
    let dimmensions = get_present_dimmensions(present).unwrap_or(vec![0,0,0]);
    let mut surface_areas : Vec<usize> = Vec::new();
    surface_areas.push(dimmensions[0]*dimmensions[1]);
    surface_areas.push(dimmensions[1]*dimmensions[2]);
    surface_areas.push(dimmensions[0]*dimmensions[2]);

    let mut smallest_side : usize = surface_areas[0];
    let mut total_surface_area : usize = 0;
    let areas_iter : IntoIter<usize> = surface_areas.into_iter();
    for area in areas_iter {
       smallest_side = if area < smallest_side { area } else { smallest_side };
       total_surface_area += area*2;
    }

    return total_surface_area + smallest_side;
}

pub fn bow_length_required(present : &String) -> usize {
    get_present_dimmensions(present).unwrap_or(vec![0,0,0])
        .into_iter()
        .fold(1, |product, curr| product * curr)
}

pub fn ribbon_length_required(present: &String) -> usize {
    let dimmensions : Vec<usize> = get_present_dimmensions(present).unwrap_or(vec![0,0,0]);
    let mut perimeters : Vec<usize> = Vec::new();
    perimeters.push(2*dimmensions[0] + 2*dimmensions[1]);
    perimeters.push(2*dimmensions[1] + 2*dimmensions[2]);
    perimeters.push(2*dimmensions[0] + 2*dimmensions[2]);
    perimeters.into_iter().min().unwrap_or(0)
}

pub fn sqft_needed() -> usize {
    BufReader::new(File::open("src/presents.txt").ok().expect("Bad file_name"))
        .lines()
        .map(|line| wrapping_paper_required(&line.ok().expect("Bad line provided in file")))
        .fold(0, |sum, curr| sum + curr)
}

pub fn ribbon_needed() -> usize {
    BufReader::new(File::open("src/presents.txt").ok().expect("Bad file_name"))
        .lines()
        .map(|line| {
                 let present = line.ok().expect("Bad line provided in file");
                 bow_length_required(&present) + ribbon_length_required(&present)
        })
        .fold(0, |sum, curr| sum + curr)
}
