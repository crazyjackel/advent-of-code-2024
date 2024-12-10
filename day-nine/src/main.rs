use std::{
    collections::HashSet,
    fs::File,
    io::{Read, SeekFrom},
    u8,
};

#[derive(Debug, PartialEq, Clone)]
enum Block {
    File(u32, u8),
    Free(u8),
}


impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File(id, len) => write!(f, "File({}) len:{}", id, len),
            Block::Free(len) => write!(f, "Free len:{}", len),
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = parse_blocks(contents);
    let fill_free_result = fill_free_3(&result);
    println!("{}", check_sum(fill_free_result));
}

fn parse_blocks(str: String) -> Vec<Block> {
    let mut push_vec = Vec::new();
    for i in str.chars().enumerate() {
        if i.0 % 2 == 0 {
            let index = i.0 as u32 / 2;
            if let Some(len) = char_to_length(i.1) {
                push_vec.push(Block::File(index, len));
            }
        } else {
            if let Some(len) = char_to_length(i.1) {
                push_vec.push(Block::Free(len));
            }
        }
    }
    push_vec
}

fn check_sum(sum: Vec<Block>) -> u64 {
    let mut check = 0u64;
    let mut index = 0;
    for (_, block) in sum.iter().enumerate() {
        match block {
            Block::Free(len) => { index += *len as u32; }
            Block::File(id, len) => {
                for i in 0..*len {
                    let real_index = index + i as u32;
                    check += real_index as u64 * *id as u64;
                }
                index += *len as u32;
            }
        }
    }
    check
}

fn get_last_file_in_bounds(
    block_parse: &Vec<Block>,
    bounds: (usize, usize),
) -> Option<(usize, &Block)> {
    for i in (bounds.0..=bounds.1).rev() {
        if let Some(file) = block_parse.get(i) {
            if let Block::File(file_length, file_len) = file {
                return Some((i,file));
            }
        }
    }
    None
}


fn fill_free_3(block_parse: &Vec<Block>) -> Vec<Block> {
    let mut swap_vector = block_parse.clone();
    let mut beginning: usize = 0; //Beginning
    let mut end: usize = block_parse.len() - 1; //End
    while end > beginning {

        //Get Last File greater between the beginning and end bounds
        for index in (beginning..=end).rev(){
            if let Block::File(file_index, file_len) = swap_vector[index] {
                //Find Free Space between the beginning and discovered index
                let mut found_free = false;
                'inner: for j in (beginning..index) {
                    if let Block::Free(block_len) = swap_vector[j] {
                        found_free = true;
                        if file_len <= block_len {
                            //We replace last file with free
                            swap_vector[index] = Block::Free(file_len);
                            //We set the open space to the last
                            swap_vector[j] = Block::File(file_index, file_len);
                            //We add additional free
                            let difference = &block_len - file_len;
                            if difference != 0 {
                                swap_vector.insert(j+1, Block::Free(difference));
                            }
                            end = index;
                            break 'inner;
                        }
                    }
                    else {
                        if !found_free && j > beginning {
                            beginning = j;
                        }
                    }
                }
                if end > 2{
                    end -= 2;
                }
            }
        }
    }
    swap_vector.into_iter().filter(|x| match x{
        Block::File(_, _) => true,
        Block::Free(len) => if len == &0u8 {false}else{true},
    }).collect()
}

// fn fill_free_2(block_parse: &Vec<Block>) -> Vec<Block> {
//     let mut push_vec = Vec::new();
//     let mut last_file_index = block_parse.len() - 1;
//     if let Block::Free(_) = block_parse.get(last_file_index).unwrap() {
//         last_file_index -= 1;
//     }

//     let mut hash_index: HashSet<u32> = HashSet::new();
//     for (index, block) in block_parse.iter().enumerate() {
//         match *block {
//             Block::File(index, len) => {
//                 if hash_index.contains(&index) {
//                     push_vec.push(Block::Free(len));
//                 } else {
//                     push_vec.push(Block::File(index, len));
//                 }
//             }
//             Block::Free(len) => {
//                 if index >= last_file_index {
//                     break;
//                 }

//                 for i in (index + 1..=last_file_index).rev() {
//                     let block = block_parse.get(i).unwrap();
//                     if let Block::File(file_index, file_length) = block {}

//                     let mut fill_space = len;
//                     while fill_space > 0 {
//                         //Get Block from the Tail end up to index
//                         let block: Option<&Block> = {
//                             let mut result = None;
//                             for i in (index + 1..=last_file_index).rev() {
//                                 let block = block_parse.get(i).unwrap();
//                                 if let Block::File(file_index, file_length) = block {
//                                     if hash_index.contains(file_index) {
//                                         continue;
//                                     }

//                                     if file_length <= &fill_space {
//                                         if i == last_file_index {
//                                             last_file_index -= 2;
//                                         }
//                                         result = Some(block);
//                                         break;
//                                     }
//                                 }
//                             }
//                             result
//                         };
//                         if let Some(Block::File(index, len)) = block {
//                             hash_index.insert(*index);
//                             push_vec.push(Block::File(*index, *len));
//                             fill_space -= len;
//                         } else {
//                             push_vec.push(Block::Free(fill_space));
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//         push_vec
//     }
// }

// fn get_first_file_from_back_of_size(block_parse: &Vec<Block>, len: u8) {}

fn fill_free(block_parse: &Vec<Block>) -> Vec<Block> {
    let mut push_vec = Vec::new();
    let mut last_ptr = block_parse.len() - 1;
    let mut remainder_block = Block::Free(0);
    let mut count_free: u32 = 0;
    for (block_index, block) in block_parse.iter().enumerate() {
        match *block {
            Block::File(index, len) => push_vec.push(Block::File(index, len)),
            Block::Free(len) => {
                if last_ptr <= block_index {
                    break;
                }
                count_free += len as u32;
                let mut fill_length = len;
                if let Block::File(index, len) = remainder_block {
                    if fill_length >= len {
                        push_vec.push(remainder_block);
                        remainder_block = Block::Free(0);
                        fill_length -= len;
                    } else {
                        push_vec.push(Block::File(index, fill_length));
                        remainder_block = Block::File(index, len - fill_length);
                        fill_length = 0;
                    }
                }

                while fill_length > 0 {
                    let block = block_parse.get(last_ptr).unwrap();
                    last_ptr -= 1;
                    if let Block::Free(len2) = *block {
                        count_free += len2 as u32;
                        continue;
                    } else if let Block::File(index, len) = *block {
                        if fill_length >= len {
                            push_vec.push(Block::File(index, len));
                            fill_length -= len;
                        } else {
                            push_vec.push(Block::File(index, fill_length));
                            remainder_block = Block::File(index, len - fill_length);
                            break;
                        }
                    }
                }
            }
        }
    }

    if let Block::File(_, _) = remainder_block {
        push_vec.push(remainder_block);
    }

    while count_free > 0 {
        if count_free > u8::MAX as u32 {
            push_vec.push(Block::Free(u8::MAX));
            count_free -= u8::MAX as u32;
        } else {
            push_vec.push(Block::Free(count_free as u8));
            count_free = 0;
        }
    }
    push_vec
}

fn char_to_length(char: char) -> Option<u8> {
    let result = match char {
        '0' => Some(0u8),
        '1' => Some(1u8),
        '2' => Some(2u8),
        '3' => Some(3u8),
        '4' => Some(4u8),
        '5' => Some(5u8),
        '6' => Some(6u8),
        '7' => Some(7u8),
        '8' => Some(8u8),
        '9' => Some(9u8),
        _ => None,
    };
    result
}

fn to_string(blocks: &Vec<Block>) -> String {
    let mut s = String::new();
    for block in blocks {
        match block {
            Block::File(index, len) => {
                for _ in 0..*len {
                    s.push_str(index.to_string().as_str());
                }
            }
            Block::Free(len) => {
                for _ in 0..*len {
                    s.push('.');
                }
            }
        }
    }
    s
}

#[test]
fn test_parse() {
    let str = "2333133121414131402".to_string();
    let result = parse_blocks(str);
    let fill_free_result = fill_free_3(&result);
    println!("{:?}", fill_free_result);
    println!("{}", check_sum(fill_free_result));
}
