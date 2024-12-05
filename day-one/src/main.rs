// Problem:
// Read in a line
// Get the sum of consecutive smallests from that file

// Improvement Notes:
// Algo may be made faster by not doing a vec sort through an early out.
// Let's image the following datum:
// [1,2,3,4,5,6] & [2,3,2,2,5,6]
// If we were to break the data up into groups of 2 and save the highest and lowest number of both:
// 1: [[1,2],[2,3]] -> difference: 2, bounds: (1-3)
// 2: [[3,4],[2,2]] -> difference: 3, bounds: (2-4)
// 3: [[5,6],[5,6]] -> difference: 0, bounds: (5-6)
// We would recognize that there is no way for group 1 or 2 to interfere with the score of group 3.
// By then recombining group 1 and 2 together and then adding it with 3, we could get the difference accurately.

use std::{fs::File, io::{self, BufRead}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut vec : Vec<i32> = Vec::new();
    let mut vec2 : Vec<i32> = Vec::new();
    for line in reader.lines().filter_map(|x| x.ok()){
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        if words.len() != 2{
            panic!("Words is not two");
        }
        vec.push(words.get(0).unwrap().parse().unwrap());
        vec2.push(words.get(1).unwrap().parse().unwrap());
    }
    vec.sort();
    vec2.sort();

    if vec.len() != vec2.len(){
        panic!("Lengths don't match");
    }

    let mut count = 0;
    for i in 0..vec.len() {
        let num1 = vec.get(i).unwrap();
        let num2 = vec2.get(i).unwrap();

        let difference = i32::abs(num1 - num2);
        println!("Running Count: {}, ({},{})", difference, num1, num2);
        count += difference;
    }
    println!("The answer is {}", count);
}
