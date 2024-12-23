use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //let mut count: u64 = 0;
    let mut sequence_map: HashMap<i32, (u16, u16)> = HashMap::new();
    for line in contents.lines() {
        let mut num: u64 = line.parse().unwrap();
        let num_1 = num;
        let num_2 = pseudo(num);
        num = num_2;
        let num_3 = pseudo(num);
        num = num_3;
        let num_4 = pseudo(num);
        num = num_4;
        let mut previous_digit: i8 = (num_4 % 10) as i8;
        let mut previous_difference: i8 = ((num_4 % 10) as i64 - (num_3 % 10) as i64) as i8;
        let mut previous_difference_1: i8 = ((num_3 % 10) as i64 - (num_2 % 10) as i64) as i8;
        let mut previous_difference_2: i8 = ((num_2 % 10) as i64 - (num_1 % 10) as i64) as i8;
        for i in 3..2000 {
            num = pseudo(num);
            let digit = (num % 10) as i8;
            let current_difference = digit - previous_digit;
            let ele = sequence_map
                .entry(encode((
                    previous_difference_2,
                    previous_difference_1,
                    previous_difference,
                    current_difference,
                )))
                .or_insert((0, 0));
            if ele.1 == 0 {
                ele.0 += digit as u16;
                ele.1 = i;
            }
            previous_difference_2 = previous_difference_1;
            previous_difference_1 = previous_difference;
            previous_difference = current_difference;
            previous_digit = digit;
        }

        for sequence in sequence_map.iter_mut() {
            *sequence.1 = (sequence.1 .0, 0);
        }
        //println!("{}", num);
        //count += num;
    }

    if let Some(ele) = sequence_map.iter().max_by(|(_, a), (_, b)| a.cmp(b)) {
        println!("{:?}", ele);
    }
}

#[test]
fn test_seqeunces() {
    let contents = "1
2
3
2024";
    let mut sequence_map: HashMap<i32, (u16, u16)> = HashMap::new();
    for line in contents.lines() {
        let mut num: u64 = line.parse().unwrap();
        let num_1 = num;
        let num_2 = pseudo(num);
        num = num_2;
        let num_3 = pseudo(num);
        num = num_3;
        let num_4 = pseudo(num);
        num = num_4;
        let mut previous_digit: i8 = (num_4 % 10) as i8;
        let mut previous_difference: i8 = ((num_4 % 10) as i64 - (num_3 % 10) as i64) as i8;
        let mut previous_difference_1: i8 = ((num_3 % 10) as i64 - (num_2 % 10) as i64) as i8;
        let mut previous_difference_2: i8 = ((num_2 % 10) as i64 - (num_1 % 10) as i64) as i8;
        for i in 3..2000 {
            num = pseudo(num);
            let digit = (num % 10) as i8;
            let current_difference = digit - previous_digit;
            let ele = sequence_map
                .entry(encode((
                    previous_difference_2,
                    previous_difference_1,
                    previous_difference,
                    current_difference,
                )))
                .or_insert((0, 0));
            if ele.1 == 0 {
                ele.0 += digit as u16;
                ele.1 = i;
            }
            previous_difference_2 = previous_difference_1;
            previous_difference_1 = previous_difference;
            previous_difference = current_difference;
            previous_digit = digit;
        }

        for sequence in sequence_map.iter_mut() {
            *sequence.1 = (sequence.1 .0, 0);
        }
        //println!("{}", num);
        //count += num;
    }

    if let Some(ele) = sequence_map.iter().max_by(|(_, (a,_)), (_, (b,_))| a.cmp(b)) {
        println!("{:?}, {:?}", decode(*ele.0), ele.1);
    }
}

fn encode(seq: (i8, i8, i8, i8)) -> i32 {
    (seq.0 + 9) as i32
        + 20 * (seq.1 + 9) as i32
        + 400 * (seq.2 + 9) as i32
        + 8000 * (seq.3 + 9) as i32
}
fn decode(num: i32) -> (i8, i8, i8, i8) {
    let d3 = (num / 8000) as i8 - 9; // Extract the fourth value
    let rem = num % 8000;
    let d2 = (rem / 400) as i8 - 9; // Extract the third value
    let rem = rem % 400;
    let d1 = (rem / 20) as i8 - 9; // Extract the second value
    let d0 = (rem % 20) as i8 - 9; // Extract the first value
    (d0, d1, d2, d3)
}

#[test]
fn encode_decode() {
    let test = (-2, 1, -1, 3);
    assert_eq!(test, decode(encode(test)));
}

fn pseudo(mut secret_number: u64) -> u64 {
    let mut secret_number_2 = secret_number * 64; //Left Bit Shift by 6
    secret_number ^= secret_number_2;
    secret_number = secret_number % 16777216; // 2^24
    secret_number_2 = secret_number / 32; //Right Bit shift by 5
    secret_number ^= secret_number_2;
    secret_number = secret_number % 16777216; // 2^24
    secret_number_2 = secret_number * 2048; //Left Bit Shift by 11
    secret_number ^= secret_number_2;
    secret_number = secret_number % 16777216; // 2^24
    secret_number
}
