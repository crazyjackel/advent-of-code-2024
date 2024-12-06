use std::{
    fs::File,
    future,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    process_file_2(reader);
}

fn process_file_1(reader: io::BufReader<File>) {
    let mut num_safe = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(String::from)
            .map(|x| x.parse().unwrap())
            .collect();

        if is_safe(&levels) {
            num_safe += 1;
        }
    }
    println!("Total Safe {}", num_safe);
}

fn process_file_2(reader: io::BufReader<File>) {
    let mut num_safe = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(String::from)
            .map(|x| x.parse().unwrap())
            .collect();

        if is_safe(&levels) {
            num_safe += 1;
        }
        else if safe_with_removal(&levels){
            num_safe += 1;
        }
    }
    println!("Total Safe {}", num_safe);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let mut num_increasing = 0;
    let mut num_decreasing = 0;
    for index in 1..levels.len() {
        let previous_level = levels.get(index - 1).unwrap();
        let current_level = levels.get(index).unwrap();
        let difference = current_level - previous_level;
        if difference == 0 || difference > 3 || difference < -3 {
            return false;
        }

        if difference > 0 {
            num_increasing += 1;
            if num_decreasing > 0 {
                return false;
            }
        }

        if difference < 0 {
            num_decreasing += 1;
            if num_increasing > 0 {
                return false;
            }
        }
    }
    true
}

fn safe_with_removal(levels: &Vec<i32>) -> bool {
    for index in 0..levels.len(){
        let mut new_levels = levels.clone();
        new_levels.remove(index);
        let is_safe = is_safe(&new_levels);
        if is_safe{
            return true;
        }
    }
    false
}

// Can handle one problem by removing a single level:
// Possibilities:
// if I find a problem, reset is_increasing if and only if it is the first index.
// Count how many are increasing? No, won't work because
// fn is_safe_dampened(levels: Vec<i32>) -> Option<bool> {
//     let mut is_increasing: Option<bool> = None;
//     for index in 1..levels.len() {
//         let previous_level = levels.get(index - 1)?;
//         let current_level = levels.get(index)?;
//         let difference = current_level - previous_level;
//         if difference == 0 || difference > 3 || difference < -3 {
//             // let mut levels_without_previous = levels.clone();
//             // levels_without_previous.remove(index - 1);
//             // let mut levels_without_current = levels.clone();
//             // levels_without_current.remove(index);
//             // let is_previous_safe = is_safe(levels_without_previous)?;
//             // let is_current_safe = is_safe(levels_without_current)?;
//             // return Some(is_previous_safe|| is_current_safe);
//         }

//         if is_increasing == None {
//             is_increasing = if difference > 0 { Some(true) } else { Some(false) };
//         }

//         if (is_increasing.unwrap_or(false) && difference < 0) || (!is_increasing.unwrap_or(true) && difference > 0){
//             // let mut levels_without_previous = levels.clone();
//             // levels_without_previous.remove(index - 1);
//             // let mut levels_without_current = levels.clone();
//             // levels_without_current.remove(index);
//             // let is_previous_safe = is_safe(levels_without_previous)?;
//             // let is_current_safe = is_safe(levels_without_current)?;
//             // return Some(is_previous_safe|| is_current_safe);
//         }
//     }
//     Some(true)
// }

#[test]
fn is_safe_tests() {
    let vecs1 = vec![7, 6, 4, 2, 1];
    let vecs2 = vec![1, 2, 7, 8, 9];
    let vecs3 = vec![9, 7, 6, 2, 1];
    let vecs4 = vec![1, 3, 2, 4, 5];
    let vecs5 = vec![8, 6, 4, 4, 1];
    let vecs6 = vec![1, 3, 6, 7, 9];

    assert_eq!(is_safe(&vecs1), true);
    assert_eq!(is_safe(&vecs2), false);
    assert_eq!(is_safe(&vecs3), false);
    assert_eq!(is_safe(&vecs4), false);
    assert_eq!(is_safe(&vecs5), false);
    assert_eq!(is_safe(&vecs6), true);
}

#[test]
fn is_safe_dampened_tests() {
    let vecs1 = vec![7, 6, 4, 2, 1];
    let vecs2 = vec![1, 2, 7, 8, 9];
    let vecs3 = vec![9, 7, 6, 2, 1];
    let vecs4 = vec![1, 3, 2, 4, 5];
    let vecs5 = vec![8, 6, 4, 4, 1];
    let vecs6 = vec![1, 3, 6, 7, 9];
    let vecs7 = vec![3, 4, 2, 1, 0];

    assert_eq!(safe_with_removal(&vecs1), true);
    assert_eq!(safe_with_removal(&vecs2), false);
    assert_eq!(safe_with_removal(&vecs3), false);
    assert_eq!(safe_with_removal(&vecs4), true);
    assert_eq!(safe_with_removal(&vecs5), true);
    assert_eq!(safe_with_removal(&vecs6), true);
    assert_eq!(safe_with_removal(&vecs7), true);
}
