use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::Read,
    usize,
};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let numeric_chars: Vec<char> = "029A".chars().collect();
    //805 -> 18931
    //964 -> 19990

    //Maps Two Numeric Characters to their shortest Sequences of Digital Characters
    let mut numeric_map: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();
    //Maps Two Digital Characters to their shortest Sequences of Digital Characters
    let mut digital_map: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();

    let mut count = 0;
    for i in 0..numeric_chars.len() {
        // EX: A
        let first_numeric_char = if i == 0 { 'A' } else { numeric_chars[i - 1] };
        // EX: 0
        let second_numeric_char = numeric_chars[i];
        let entry = numeric_map
            .entry((first_numeric_char, second_numeric_char))
            .or_insert_with_key(|x| expand_code(x.0, x.1, &get_neighbor_numeric));

        let mut minimum: usize = usize::MAX;
        for res in &entry.1 {
            let mut memoization: HashMap<SequenceInput, usize> = HashMap::new();
            let length = get_shortest_sequence_length_digital(
                &SequenceInput {
                    key: res.to_vec(),
                    iteration: 4,
                },
                &mut digital_map,
                &mut memoization,
            );
            if length < minimum {
                minimum = length;
            }
        }
        count += minimum;
    }
    println!("{}", count);
}

fn get_neighbor_digital(character: char) -> HashMap<char, char> {
    match character {
        'A' => vec![('>', 'v'), ('^', '<')],
        '>' => vec![('A', '^'), ('v', '<')],
        '<' => vec![('v', '>')],
        '^' => vec![('A', '>'), ('v', 'v')],
        'v' => vec![('<', '<'), ('^', '^'), ('>', '>')],
        _ => vec![],
    }
    .into_iter()
    .collect()
}

fn get_neighbor_numeric(character: char) -> HashMap<char, char> {
    match character {
        'A' => vec![('0', '<'), ('3', '^')],
        '0' => vec![('2', '^'), ('A', '>')],
        '1' => vec![('2', '>'), ('4', '^')],
        '2' => vec![('0', 'v'), ('3', '>'), ('1', '<'), ('5', '^')],
        '3' => vec![('A', 'v'), ('2', '<'), ('6', '^')],
        '4' => vec![('1', 'v'), ('5', '>'), ('7', '^')],
        '5' => vec![('2', 'v'), ('6', '>'), ('4', '<'), ('8', '^')],
        '6' => vec![('5', '<'), ('9', '^'), ('3', 'v')],
        '7' => vec![('4', 'v'), ('8', '>')],
        '8' => vec![('7', '<'), ('5', 'v'), ('9', '>')],
        '9' => vec![('8', '<'), ('6', 'v')],
        _ => vec![],
    }
    .into_iter()
    .collect()
}

fn expand_code(
    first_char: char,
    second_char: char,
    get_neighbors: &dyn Fn(char) -> HashMap<char, char>,
) -> (usize, Vec<Vec<char>>) {
    // If the start and end characters are the same, return the trivial path
    if first_char == second_char {
        return (1, vec![vec!['A']]);
    }

    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut min_path_length = usize::MAX;

    queue.push_back((first_char, vec![]));
    while let Some((current_char, path)) = queue.pop_front() {
        if path.len() > min_path_length {
            continue;
        }
        let neighbors = get_neighbors(current_char);
        for (&neighbor, &direction) in &neighbors {
            let mut new_path = path.clone();
            new_path.push(direction);

            if neighbor == second_char {
                new_path.push('A');
                let path_length = new_path.len();

                // If this is a new shorter path, clear previous results and update min_path_length
                if path_length < min_path_length {
                    paths.clear();
                    min_path_length = path_length;
                }

                // Add the path if it matches the shortest length
                if path_length == min_path_length {
                    paths.push(new_path);
                }
            } else {
                queue.push_back((neighbor, new_path));
            }
        }
    }
    (min_path_length, paths)
}
// fn expand_code_numeric(first_char: char, second_char: char) -> Vec<Vec<char>>{
//     if first_char == second_char{
//         return vec![vec!['A']];
//     }
//     let mut vec: Vec<Vec<char>> = Vec::new();
//     let map = get_neighbor_numeric(first_char);
//     if map.contains_key(&second_char){
//         vec.push(vec![map[&second_char], 'A']);
//     }else{
//         for key in map.keys(){
//             let expanded_code = expand_code_numeric(*key, second_char);
//             for code in expanded_code{
//                 let mut new_vec = vec![map[key]];
//                 new_vec.extend(code);
//                 vec.push(new_vec);
//             }
//         }
//     }
//     vec
// }

#[test]
fn test_code_expansion() {
    println!("{:?}", expand_code('A', '0', &get_neighbor_numeric));
    println!("{:?}", expand_code('0', '2', &get_neighbor_numeric));
    println!("{:?}", expand_code('2', '9', &get_neighbor_numeric));
    println!("{:?}", expand_code('9', 'A', &get_neighbor_numeric));
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct SequenceInput {
    key: Vec<char>,
    iteration: u8,
}

fn get_shortest_sequence_length_digital(
    key: &SequenceInput,
    digital_map: &mut HashMap<(char, char), (usize, Vec<Vec<char>>)>,
    memoization: &mut HashMap<SequenceInput, usize>,
) -> usize {
    if memoization.contains_key(key) {
        return memoization[key];
    }

    if key.iteration == 1 {
        let k1 = key.clone();
        let state = memoization.entry(k1).or_insert_with_key(|x| {
            let mut count: usize = 0;
            for i in 0..x.key.len() {
                let first_digital_char = if i == 0 { 'A' } else { x.key[i - 1] };
                let second_digital_char = x.key[i];
                let result = digital_map
                    .entry((first_digital_char, second_digital_char))
                    .or_insert_with_key(|ch| expand_code(ch.0, ch.1, &get_neighbor_digital));
                count += result.0;
            }
            count
        });
        return *state;
    }

    let mut count: usize = 0;
    for i in 0..key.key.len() {
        let first_digital_char = if i == 0 { 'A' } else { key.key[i - 1] };
        let second_digital_char = key.key[i];
        let result = digital_map
            .entry((first_digital_char, second_digital_char))
            .or_insert_with_key(|ch| expand_code(ch.0, ch.1, &get_neighbor_digital))
            .clone();

        let mut minimum: usize = usize::MAX;
        for res in result.1 {
            let length = get_shortest_sequence_length_digital(
                &SequenceInput {
                    iteration: key.iteration - 1,
                    key: res,
                },
                digital_map,
                memoization,
            );
            if length < minimum {
                minimum = length;
            }
        }
        // memoization.insert(key.clone(), count);
        // Algo was broken cause of this: f*ck. Moved it to proper place
        count += minimum;
    }
    memoization.insert(key.clone(), count);
    count
}

#[test]
fn get_shortest_sequence() {
    let numeric_chars: Vec<char> = "671A".chars().collect();

    // 805A -> 86475783012
    // 964A -> 85006969638
    // 459A -> 90594397580
    // 968A -> 86475783010
    // 671A -> 90750571882


    //Maps Two Numeric Characters to their shortest Sequences of Digital Characters
    let mut numeric_map: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();
    //Maps Two Digital Characters to their shortest Sequences of Digital Characters
    let mut digital_map: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();
    let mut memoization: HashMap<SequenceInput, usize> = HashMap::new();

    let mut count = 0;
    for i in 0..numeric_chars.len() {
        // EX: A
        let first_numeric_char = if i == 0 { 'A' } else { numeric_chars[i - 1] };
        // EX: 0
        let second_numeric_char = numeric_chars[i];
        let entry = numeric_map
            .entry((first_numeric_char, second_numeric_char))
            .or_insert_with_key(|x| expand_code(x.0, x.1, &get_neighbor_numeric));

        let mut minimum: usize = usize::MAX;
        for res in &entry.1 {
            let length = get_shortest_sequence_length_digital(
                &SequenceInput {
                    key: res.to_vec(),
                    iteration: 25,
                },
                &mut digital_map,
                &mut memoization,
            );
            if length < minimum {
                minimum = length;
            }
        }
        count += minimum;
    }

    println!("{:?}", digital_map);
    println!("");
    println!("{:?}", memoization);

    println!("{}", count);
}

//805A

// All Sequences Start and end at A
// 0 2
// ^ A
// < A > A
// > > ^ A v A ^ A

// //Maps Two Numeric Characters to their shortest Sequences of Digital Characters after being mapped to digital and digital again.
// let mut numeric_to_digital_map: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();
// //Maps Two Numeric Characters to their shortest Sequences of Digital Characters after being mapped to digital and digital and digital again.
// let mut numeric_to_digital_to_digital: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();
// //Maps Two Numeric Characters to their shortest Sequences of Digital Characters after being mapped to digital and digital and digital again.
// let mut numeric_to_digital_to_digital_to_digital: HashMap<(char, char), (usize, Vec<Vec<char>>)> = HashMap::new();

// // EX: (2, [['<','A']])
// let map_value = &numeric_map[&(first_numeric_char, second_numeric_char)];
// let mut stack : VecDeque<CodeState> = VecDeque::new();
// let mut count: usize = 0;
// while let Some(back_state) = stack.pop_back(){
//     if back_state.iteration == 0{
//         count += back_state.code_length;
//         continue;
//     }
// }

//println!("{}", numeric_map[&(first_numeric_char, second_numeric_char)].0);
//println!("{:?}: {:?}", (first_numeric_char, second_numeric_char), map_value);
// for digital_char in &map_value.1 {
//     let mut sequence_size: usize = 0;

//     let mut full_vec: Vec<Vec<char>> = Vec::new();
//     for i in 0..digital_char.len() {
//         let first_digital_char = if i == 0 { 'A' } else { digital_char[i - 1] };
//         let second_digital_char = digital_char[i];
//         if !digital_map.contains_key(&(first_digital_char, second_digital_char)) {
//             let result = expand_code(
//                 first_digital_char,
//                 second_digital_char,
//                 &get_neighbor_digital,
//             );
//             digital_map.insert((first_digital_char, second_digital_char), result);
//         };
//         let digital = &digital_map[&(first_digital_char, second_digital_char)];
//         full_vec = merge_sequences(&full_vec, &digital.1);
//         sequence_size += digital.0;
//     }

//     let entry = numeric_to_digital_map
//         .entry((first_numeric_char, second_numeric_char))
//         .or_insert((usize::MAX, Vec::new()));
//     if sequence_size < entry.0{
//         entry.0 = sequence_size;
//         entry.1.clear();
//     }
//     entry.1.append(&mut full_vec);
// }

// let map_value = &numeric_to_digital_map[&(first_numeric_char, second_numeric_char)];
// //println!("{}", numeric_to_digital_map[&(first_numeric_char, second_numeric_char)].0);
// //println!("{:?}: {:?}", (first_numeric_char, second_numeric_char), map_value);
// for digital_char in &map_value.1 {
//     let mut sequence_size: usize = 0;

//     let mut full_vec: Vec<Vec<char>> = Vec::new();
//     for i in 0..digital_char.len() {
//         let first_digital_char = if i == 0 { 'A' } else { digital_char[i - 1] };
//         let second_digital_char = digital_char[i];
//         if !digital_map.contains_key(&(first_digital_char, second_digital_char)) {
//             let result = expand_code(
//                 first_digital_char,
//                 second_digital_char,
//                 &get_neighbor_digital,
//             );
//             digital_map.insert((first_digital_char, second_digital_char), result);
//         };
//         let digital = &digital_map[&(first_digital_char, second_digital_char)];
//         full_vec = merge_sequences(&full_vec, &digital.1);
//         sequence_size += digital.0;
//     }

//     let entry = numeric_to_digital_to_digital
//         .entry((first_numeric_char, second_numeric_char))
//         .or_insert((usize::MAX, Vec::new()));
//     if sequence_size < entry.0{
//         entry.0 = sequence_size;
//         entry.1.clear();
//     }
//     entry.1.append(&mut full_vec);
// }

// println!("{}", numeric_to_digital_to_digital[&(first_numeric_char, second_numeric_char)].0);

// let map_value = &numeric_to_digital_to_digital[&(first_numeric_char, second_numeric_char)];
// //println!("{:?}: {:?}", (first_numeric_char, second_numeric_char), map_value);
// for digital_char in &map_value.1 {
//     let mut sequence_size: usize = 0;

//     let mut full_vec: Vec<Vec<char>> = Vec::new();
//     for i in 0..digital_char.len() {
//         let first_digital_char = if i == 0 { 'A' } else { digital_char[i - 1] };
//         let second_digital_char = digital_char[i];
//         if !digital_map.contains_key(&(first_digital_char, second_digital_char)) {
//             let result = expand_code(
//                 first_digital_char,
//                 second_digital_char,
//                 &get_neighbor_digital,
//             );
//             digital_map.insert((first_digital_char, second_digital_char), result);
//         };
//         let digital = &digital_map[&(first_digital_char, second_digital_char)];
//         full_vec = merge_sequences(&full_vec, &digital.1);
//         sequence_size += digital.0;
//     }

//     let entry = numeric_to_digital_to_digital_to_digital
//         .entry((first_numeric_char, second_numeric_char))
//         .or_insert((usize::MAX, Vec::new()));
//     if sequence_size < entry.0{
//         entry.0 = sequence_size;
//         entry.1.clear();
//     }
//     entry.1.append(&mut full_vec);
// }

// println!("{}", numeric_to_digital_to_digital_to_digital[&(first_numeric_char, second_numeric_char)].0);

// //Maps Two Numeric Characters to their longest first Digital Sequence
// let map = to_map(&str, &get_neighbor_numeric);

// let mut map2: HashMap<(char, char), Vec<Vec<char>>> =
//     map.iter().fold(HashMap::new(), |mut acc, ele| {
//         for vec in ele.1{
//             let inner_map = to_map(vec, &get_neighbor_digital);
//         }
//         acc
//     });

// fn merge_sequences(
//     vec1: &Vec<Vec<char>>,
//     vec2: &Vec<Vec<char>>,
// ) -> Vec<Vec<char>> {
//     if vec1.is_empty(){
//         return vec2.clone();
//     }
//     vec1.clone().into_iter()
//         .flat_map(|seq1| {
//             vec2.iter().map(move |seq2| {
//                 let mut merged = seq1.clone();
//                 merged.extend(seq2.iter());
//                 merged
//             })
//         })
//         .collect()
// }
// fn get_length<T, A>(map: HashMap<T, Vec<Vec<A>>>) -> usize {
//     map.iter().fold(0, |mut acc, ele| {
//         if let Some(ele2) = ele.1.first() {
//             acc += ele2.len();
//         }
//         acc
//     })
// }

// fn to_map(
//     str: &Vec<char>,
//     get_neighbors: &dyn Fn(char) -> HashMap<char, char>,
// ) -> HashMap<(char, char), Vec<Vec<char>>> {
//     let mut map: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
//     for i in 0..str.len() {
//         let char_1 = if i == 0 { 'A' } else { str[i - 1] };
//         let char_2 = str[i];
//         if !map.contains_key(&(char_1, char_2)) {
//             let result = expand_code(char_1, char_2, get_neighbors);
//             map.insert((char_1, char_2), result);
//         };
//     }
//     map
// }
