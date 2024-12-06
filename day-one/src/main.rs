

use std::{
    collections::HashMap, fs::File, io::{self, BufRead}
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    process_file_2(reader);
}

// Problem:
// Read in a line
// Get the sum of consecutive smallests from that file

// Improvement Notes:
// Algo may be made faster by not doing a vec sort (or minimizing size of sorts) through an early out.
// Let's image the following datum:
// [1,2,3,4,5,6] & [2,3,2,2,5,6]
// If we were to break the data up into groups of 2 and save the highest and lowest number of both:
// 1: [[1,2],[2,3]] -> difference: 2, bounds: (1-3)
// 2: [[3,4],[2,2]] -> difference: 3, bounds: (2-4)
// 3: [[5,6],[5,6]] -> difference: 0, bounds: (5-6)
// We would recognize that there is no way for group 1 or 2 to interfere with the score of group 3.
// By then recombining group 1 and 2 together and then adding it with 3, we could get the difference accurately.
// Would require an additional 2 i32 for bounds and extra time used for checking min and maxes as difference scores are processed.
// Much improved best case, makes worse case bad.
fn process_file_1(reader: io::BufReader<File>) {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    for line in reader.lines().filter_map(|x| x.ok()) {
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        if words.len() != 2 {
            panic!("Words is not two");
        }
        left_vec.push(words.get(0).unwrap().parse().unwrap());
        right_vec.push(words.get(1).unwrap().parse().unwrap());
    }
    left_vec.sort();
    right_vec.sort();

    if left_vec.len() != right_vec.len() {
        panic!("Lengths don't match");
    }

    let mut count = 0;
    for i in 0..left_vec.len() {
        let num1 = left_vec.get(i).unwrap();
        let num2 = right_vec.get(i).unwrap();

        let difference = i32::abs(num1 - num2);
        println!("Running Count: {}, ({},{})", count, num1, num2); //Can comment out used for validation
        count += difference;
    }
    println!("The answer is {}", count);
}



//Possible Improvements:
//
fn process_file_2(reader: io::BufReader<File>) {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    for line in reader.lines().filter_map(|x| x.ok()) {
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        if words.len() != 2 {
            panic!("Words is not two");
        }
        left_vec.push(words.get(0).unwrap().parse().unwrap());
        right_vec.push(words.get(1).unwrap().parse().unwrap());
    }
    let mut hashmap : HashMap<i32, i32> = HashMap::new();
    for item in right_vec.iter(){
        let entry = hashmap.entry(*item).or_insert(0);
        *entry += 1;
    }

    let mut count = 0;
    for item in left_vec.iter(){
        if let Some(num) = hashmap.get(item) {
            println!("Running Count: {}, ({},{})", count, item, num); //Can comment out, used to validate
            count += item * num;
        }
    }
    println!("The answer is {}", count);
}