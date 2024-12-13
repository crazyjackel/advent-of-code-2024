use core::hash;
use std::{
    collections::HashMap, fs::File, io::{self, BufRead, Read}, usize
};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let strs: Vec<&str> = contents.split_whitespace().collect();
    let init_vec :Vec<u64> = strs.iter().filter_map(|x| x.parse::<u64>().ok()).collect();
    let mut count = 0;
    let mut hash_map : HashMap<(u64, u8), usize>= HashMap::new();
    for v in init_vec{
        count += length_after_n_iterations(v, 75, &mut hash_map);
    }

    println!("Count {}", count);
}


fn length_after_n_iterations(number: u64, iterations: u8, hash_cache : &mut HashMap<(u64, u8), usize>) -> usize
{
    //If there is no iterations, the length is the number, so return 1
    if iterations == 0{
        return 1;
    }

    //If we already know from cache, use cache result
    if let Some(hash_result) = hash_cache.get(&(number,iterations)){
        return *hash_result;
    }

    //If iteration is one, then answer whether we split or not
    if iterations == 1
    {
        //Handle 0 case as ilog10() panics at 0... ideally preparing the hash_map AOT removes the check for a speed up 
        if number == 0{
            hash_cache.insert((number, iterations), 1);
            return 1;
        }
        let num_digits = number.ilog10() + 1;
        let len = if num_digits % 2 == 0 { 2} else {1};
        hash_cache.insert((number, iterations), len);
        return len;
    }
    else
    {
        //0 case, check length_after_n_iterations for 1 case
        if number == 0{
            let num = length_after_n_iterations(1, iterations - 1, hash_cache);
            hash_cache.insert((number, iterations), num);
            return num;
        }

        //Split Stone case, Check both lengths and add them together
        let num_digits = number.ilog10() + 1;
        if num_digits % 2 == 0{
            let factor = 10u64.pow(num_digits / 2);
            let left = number / factor;
            let right = number - (left * factor);
            
            let left_length =length_after_n_iterations(left, iterations - 1, hash_cache);
            let right_length = length_after_n_iterations(right, iterations - 1, hash_cache);
            hash_cache.insert((number, iterations), left_length + right_length);
            return left_length + right_length;
        }

        //Any remaining case, multiply by 2024 and get iterations -1
        let num = length_after_n_iterations(number * 2024, iterations - 1, hash_cache);
        hash_cache.insert((number, iterations), num);
        return num;
    }
}


#[test]
fn test_blinks(){
    let init_vec = vec![125,17];
    let mut count = 0;
    let mut hash_map : HashMap<(u64, u8), usize>= HashMap::new();
    for v in init_vec{
        count += length_after_n_iterations(v, 25, &mut hash_map);
    }

    println!("Count {}", count);
}