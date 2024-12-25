use std::{fs::File, io::Read};


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut pins : Vec<[u8;5]> = Vec::new();
    let mut locks : Vec<[u8;5]> = Vec::new();

    for structure in contents.split("\n\n"){
        let struct_lines : Vec<&str> = structure.lines().collect();
        let mut counter: [u8;5] = [0;5];
        for line in &struct_lines{
            for i in 0..line.len(){
                if &line[i..i+1] == "#"{
                    counter[i] += 1;
                }
            }
        }

        for ele in counter.iter_mut(){
            *ele -= 1;
        }

        if struct_lines[0] == "#####"{
            locks.push(counter);
        }else{
            pins.push(counter);
        }
    }

    let mut unique_combos: u64 = 0;
    for pin in &pins{
        for lock in &locks{
            let mut is_unique = true;
            for i in 0..5{
                if pin[i] + lock[i] > 5{
                    is_unique = false;
                    break;
                }
            }

            if is_unique{
                unique_combos += 1;
            }
        }
    }
    println!("{}", unique_combos)
}
