use core::num;
use std::{fs::File, io::Read, process::Output, string};

fn main() {


    let mut base_number: u64 = 0;
    let program = vec![2u8, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];
    recurse(&mut base_number, &program, program.len() - 1);
    println!("{:?}", base_number);

    let begin: u64 = 0o6500000000000000;
    let end: u64 = 0o6600000000000000;
    let program = vec![2u8, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];
    
    'outer: for octal in begin..end {
        let mut octal_2 = octal;
        for i in 0..program.len() {
            if (((octal_2 % 8) ^ 3) ^ 5) ^ ((octal_2 / (2u64.pow((octal_2 as u32 % 8) ^ 3))) % 8)
                != (program[i] as u64)
            {
                continue 'outer;
            } else {
                octal_2 /= 8;
                if octal_2 < 10000000 {
                    println!("{} {}", octal, octal_2);
                }
            }
        }
        println!("{}", octal);
        break;
    }

    // let mut file = File::open("input.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();

    // let mut register_a: u64 = 0;
    // let mut register_b: u64 = 0;
    // let mut register_c: u64 = 0;

    // let strings: Vec<&str> = contents.split_ascii_whitespace().into_iter().collect();
    // register_a = strings[2].parse().unwrap();
    // register_b = strings[5].parse().unwrap();
    // register_c = strings[8].parse().unwrap();

    // let program: Vec<u8> = strings[10]
    //     .split(',')
    //     .into_iter()
    //     .map(|x| x.parse::<u8>().unwrap())
    //     .collect();

    // let mut base_number: u64 = 0;
    // for i in (1..program.len()).rev()
    // {
    //     for k in 0..9{
    //         register_a = base_number + k * 8u64.pow(i as u32);
    //         register_b = 0;
    //         register_c = 0;
    //         let output = run_program(&mut register_a, &mut register_b, &mut register_c, &program);
    //         if output.len() == program.len(){
    //             if output[i..] == program[i..]{
    //                 base_number += k * 8u64.pow(i as u32);
    //                 println!("{} {}", i, k);
    //                 break;
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", base_number);

    // register_a = 216147450396672;
    // register_b = 0;
    // register_c = 0;
    // let output = run_program(&mut register_a, &mut register_b, &mut register_c, &program);
    // println!("{:?}", output);

    // //0,2,5,0,0,3,6,4,7
    // //6,7,5,2,1,3,5,1,7
}

fn recurse(base_number: &mut u64, program: &Vec<u8>, i: usize) {
    if i == 0{
        return;
    }
    for k in 0..8{
        let register_a = *base_number + k * 8u64.pow(i as u32);
        let output = run_2( register_a);
        if output.len() == program.len(){
            if output[i..] == program[i..]{
                *base_number += k * if i == 1 { 1} else {8u64.pow(i as u32)};
                if i < 3{
                    println!();
                }
                print!("|{} {}| ", i, k);
                if i > 0{
                    recurse(&mut base_number.clone(), program, i-1);
                }
                println!();
            }
        }
    }
}

#[test]
fn test() {
    // let begin: u64 = 216802137021380;
    // let end: u64 = 236051106700284;

    let begin: u64 = 216549846239616 - 10000;
    let end: u64 = 216549846239616 + 10000;
    let program = vec![2u8, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];
    for octal in begin..end {
        let output = run_2(octal);

        if octal % 100000 == 0 {
            println!("{}", (octal - begin) as f64 / (end - begin) as f64)
        }
        if output == program {
            println!("{}", octal);
            assert!(false);
        }
    }
    assert!(false);
}

#[test]
fn test_32() {
    let program = vec![2u8, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];
    for i in 0..program.len() {
        for digit in 1..8 {
            for digit_2 in 0..8 {
                let index = 8u64.pow((program.len() - i - 1) as u32);
                let check = if i < program.len() - 1 {
                    (digit * index) + digit_2
                } else {
                    digit_2
                };
                let calculate = calculate(check);
                if calculate == program[i] as u64 {
                    print!("({} {} {})", check, digit, digit_2);
                }
            }
        }
        println!();
    }
}

#[test]
fn test_64O() {
    let result = run_2(216549846240877);
    println!("{:?}", result);
    assert!(false);
}

#[test]
fn test_with_constraint() {
    let valid_1_or_5 = [1u8, 5]; // Allowed digits for [1 or 5]
    let valid_2_or_5 = [2, 5]; // Allowed digits for [2 or 5]

    // Iterate through all possibilities for ? and constrained positions
    for first_constraint in valid_1_or_5 {
        for second_constraint in valid_2_or_5 {
            for unknown1 in 0..8 {
                // ? can be 0-7
                for unknown2 in 0..8 {
                    // ? can be 0-7
                    let digits = [
                        6,
                        first_constraint,
                        second_constraint,
                        2,
                        7,
                        unknown1,
                        5,
                        6,
                        2,
                        7,
                        2,
                        1,
                        5,
                        7,
                        unknown2,
                        4,
                    ];

                    let octal_number = digits.iter().fold(0_u64, |acc, &d| acc * 8 + d as u64);
                    println!("{}: {:?}", octal_number, run_2(octal_number));
                }
            }
        }
    }
}

fn calculate(register_a: u64) -> u64 {
    (((register_a % 8) ^ 3) ^ 5) ^ ((register_a / (2u64.pow((register_a as u32 % 8) ^ 3))) % 8)
}

fn run_2(mut register_a: u64) -> Vec<u8> {
    let mut output = Vec::new();
    let mut register_b = 0u64;
    let mut register_c = 0u64;
    while register_a > 0 {
        register_b = (register_a % 8) ^ 3;
        register_c = register_a / 2u64.pow(register_b as u32);
        register_b ^= 5;
        register_a /= 8;
        register_b = register_b ^ register_c;
        output.push((register_b % 8) as u8);
    }
    output
}

fn run_program(
    register_a: &mut u64,
    register_b: &mut u64,
    register_c: &mut u64,
    program: &Vec<u8>,
) -> Vec<u8> {
    let mut instruction_ptr = 0;
    let mut output: Vec<u8> = Vec::new();
    while let Some(op_code) = program.get(instruction_ptr) {
        match op_code {
            0 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    let combo_operand = match operand {
                        0..=3 => (operand % 8).into(),
                        4 => *register_a % 8,
                        5 => *register_b % 8,
                        6 => *register_c % 8,
                        _ => {
                            break;
                        }
                    } as u32;
                    *register_a /= 2_u64.pow(combo_operand);
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            1 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    *register_b ^= *operand as u64;
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            2 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    *register_b = match operand {
                        0..=3 => (operand % 8).into(),
                        4 => *register_a % 8,
                        5 => *register_b % 8,
                        6 => *register_c % 8,
                        _ => {
                            break;
                        }
                    };
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            3 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    if register_a != &0 {
                        instruction_ptr = *operand as usize;
                    } else {
                        instruction_ptr += 2;
                    }
                } else {
                    break;
                }
            }
            4 => {
                if let Some(_) = program.get(instruction_ptr + 1) {
                    *register_b ^= *register_c;
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            5 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    let value: u8 = match operand {
                        0..=3 => (operand % 8).into(),
                        4 => (*register_a % 8) as u8,
                        5 => (*register_b % 8) as u8,
                        6 => (*register_c % 8) as u8,
                        _ => {
                            break;
                        }
                    };
                    output.push(value);
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            6 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    let combo_operand = match operand {
                        0..=3 => (operand % 8).into(),
                        4 => *register_a % 8,
                        5 => *register_b % 8,
                        6 => *register_c % 8,
                        _ => {
                            break;
                        }
                    } as u32;
                    *register_b = *register_a / 2_u64.pow(combo_operand);
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            7 => {
                if let Some(operand) = program.get(instruction_ptr + 1) {
                    let combo_operand = match operand {
                        0..=3 => (operand % 8).into(),
                        4 => *register_a % 8,
                        5 => *register_b % 8,
                        6 => *register_c % 8,
                        _ => {
                            break;
                        }
                    } as u32;
                    *register_c = *register_a / 2_u64.pow(combo_operand);
                    instruction_ptr += 2;
                } else {
                    break;
                }
            }
            _ => {
                instruction_ptr += 2;
            }
        }
    }
    output
}

#[test]
fn test_program() {
    let program: Vec<u8> = vec![0, 1, 5, 4, 3, 0];
    let output = run_program(&mut 729, &mut 0, &mut 0, &program);
    let str_nums = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", str_nums);
}

#[test]
fn test_1() {
    let program: Vec<u8> = vec![5, 0, 5, 1, 5, 4];
    let output = run_program(&mut 10, &mut 0, &mut 0, &program);
    assert_eq!(output, vec![0, 1, 2]);
}

#[test]
fn test_2() {
    let program: Vec<u8> = vec![0, 1, 5, 4, 3, 0];
    let output = run_program(&mut 2024, &mut 0, &mut 0, &program);
    assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
}

#[test]
fn test_3() {
    let mut b: u64 = 0;
    let program: Vec<u8> = vec![2, 6];
    let _ = run_program(&mut 0, &mut b, &mut 9, &program);
    assert_eq!(1, b);
}

#[test]
fn test_4() {
    let mut b: u64 = 29;
    let program: Vec<u8> = vec![1, 7];
    let _ = run_program(&mut 0, &mut b, &mut 0, &program);
    assert_eq!(26, b);
}

#[test]
fn test_5() {
    let mut b: u64 = 2024;
    let program: Vec<u8> = vec![4, 0];
    let _ = run_program(&mut 0, &mut b, &mut 43690, &program);
    assert_eq!(44354, b);
}

#[test]
fn output_program() {
    let program: Vec<u8> = vec![0, 3, 5, 4, 3, 0];
    let output = run_program(&mut 117440, &mut 0, &mut 0, &program);
    assert_eq!(output, program);
}
