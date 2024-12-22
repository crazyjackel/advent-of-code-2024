use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.lines().collect();
    let patterns: HashSet<&str> =
        HashSet::from_iter(lines[0].split(|x: char| x.is_whitespace() || x == ','));
    let mut count = 0;
    for line_index in 2..lines.len() {
        let line = lines[line_index];
        let constructions =
            slice_constructions(line, &patterns, &mut HashSet::new(), &mut HashMap::new());
        if constructions != 0 {
            println!("{} is constructible", line);
            count += constructions;
        }
    }
    println!("{}", count);
}

#[test]
fn test() {
    let patterns = HashSet::from_iter(vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
    let slice = "brwrr";
    let slice_2 = "bggr";
    let slice_3 = "gbbr";
    let slice_4 = "rrbgbr";
    let slice_5 = "ubwu";
    let slice_6 = "bwurrg";
    let slice_7 = "brgr";
    let slice_8 = "bbrgwb";

    assert_eq!(
        true,
        slice_is_constructible(slice, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        true,
        slice_is_constructible(slice_2, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        true,
        slice_is_constructible(slice_3, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        true,
        slice_is_constructible(slice_4, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        false,
        slice_is_constructible(slice_5, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        true,
        slice_is_constructible(slice_6, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        true,
        slice_is_constructible(slice_7, &patterns, &mut HashSet::new())
    );
    assert_eq!(
        false,
        slice_is_constructible(slice_8, &patterns, &mut HashSet::new())
    );
}

#[test]
fn test_constructions() {
    let patterns = HashSet::from_iter(vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
    let slice = "brwrr";
    let slice_2 = "bggr";
    let slice_3 = "gbbr";
    let slice_4 = "rrbgbr";
    let slice_5 = "ubwu";
    let slice_6 = "bwurrg";
    let slice_7 = "brgr";
    let slice_8 = "bbrgwb";

    assert_eq!(
        2,
        slice_constructions(slice, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        1,
        slice_constructions(slice_2, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        4,
        slice_constructions(slice_3, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        6,
        slice_constructions(slice_4, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        0,
        slice_constructions(slice_5, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        1,
        slice_constructions(slice_6, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        2,
        slice_constructions(slice_7, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
    assert_eq!(
        0,
        slice_constructions(slice_8, &patterns, &mut HashSet::new(), &mut HashMap::new())
    );
}

fn slice_is_constructible<'a>(
    slice: &'a str,
    patterns: &HashSet<&str>,
    memoization: &mut HashSet<&'a str>,
) -> bool {
    for pattern in patterns {
        if pattern.len() == 0 {
            continue;
        }
        if pattern.len() > slice.len() {
            continue;
        }
        if &&slice[..pattern.len()] == pattern {
            let new_slice = &slice[pattern.len()..];
            if new_slice == "" {
                return true;
            } else if memoization.contains(new_slice) {
            } else {
                let constructible = slice_is_constructible(new_slice, patterns, memoization);
                if constructible {
                    return true;
                } else {
                    memoization.insert(new_slice);
                }
            }
        }
    }
    return false;
}

fn slice_constructions<'a>(
    slice: &'a str,
    patterns: &HashSet<&str>,
    memoization: &mut HashSet<&'a str>,
    memoization_2: &mut HashMap<&'a str, u64>,
) -> u64 {
    let mut constructions: u64 = 0;
    for pattern in patterns {
        if pattern.len() == 0 {
            continue;
        }
        if pattern.len() > slice.len() {
            continue;
        }
        if &&slice[..pattern.len()] == pattern {
            let new_slice = &slice[pattern.len()..];
            if new_slice == "" {
                constructions += 1;
            } else if memoization.contains(new_slice) {
            } else if memoization_2.contains_key(new_slice) {
                constructions += memoization_2[new_slice];
            } else {
                let constructible =
                    slice_constructions(new_slice, patterns, memoization, memoization_2);
                if constructible != 0 {
                    constructions += constructible;
                    memoization_2.insert(&new_slice, constructible);
                } else {
                    memoization.insert(new_slice);
                }
            }
        }
    }
    constructions
}
