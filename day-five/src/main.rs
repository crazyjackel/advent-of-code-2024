use std::{
    cmp::Ordering, collections::{HashMap, HashSet}, fs::File, io::{self, BufRead}, num::ParseIntError, ops::Index
};

#[derive(PartialEq, Debug)]
struct SequenceRule {
    before_int: u32, //Integer that must be Before the after int in a sequence
    after_int: u32,  //Integer that must be After the before int in a sequence
}

#[derive(PartialEq, Debug)]
enum SequenceRuleParseError {
    InvalidSplit,
    LeftValueFailParse,
    RightValueFailParse,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let count = process_reader(reader);
    println!("Count is {}", count)
}

fn process_reader(reader: io::BufReader<File>) -> u32 {
    let mut in_first_section = true;
    let mut prior_rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut count = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
        if line.is_empty() {
            in_first_section = false;
        }

        if in_first_section {
            let result = parse_rule(line).unwrap();
            let entry = prior_rule_map.entry(result.before_int).or_insert(HashSet::new());
            entry.insert(result.after_int);
        } else {
            let values: Result<Vec<u32>, ParseIntError> = line
                .split(',')
                .into_iter()
                .map(|x| x.parse::<u32>())
                .collect();
            if let Ok(val) = values {
                if is_well_ordered(&val, &prior_rule_map) {
                    let middle_index = val.len() / 2;
                    let middle_value = val.get(middle_index).unwrap();
                    //count += middle_value;
                }
                else{
                    let mut sorted : Vec<u32> = val.clone();
                    sorted.sort_by(|a,b| compare_with_rules(a, b, &prior_rule_map));
                    let middle_index = sorted.len() / 2;
                    let middle_value = sorted.get(middle_index).unwrap();
                    count += middle_value;
                }
            }
        }
    }
    count
}

fn compare_with_rules(a : &u32, b : &u32, rule_map: &HashMap<u32, HashSet<u32>>) -> Ordering{
    let set = rule_map.get(a).unwrap();
    if set.contains(b){
        return Ordering::Less;
    }
    Ordering::Greater
}

fn is_well_ordered(vec: &Vec<u32>, rule_map: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            let num1 = vec.get(i).unwrap();
            let num2 = vec.get(j).unwrap();
            let set = rule_map.get(num1).unwrap();
            if !set.contains(num2) {
                return false;
            }
        }
    }
    true
}

fn parse_rule(rule_string: String) -> Result<SequenceRule, SequenceRuleParseError> {
    let values: Vec<&str> = rule_string.split('|').collect();
    if values.len() != 2 {
        return Err(SequenceRuleParseError::InvalidSplit);
    }
    //29,A,73
    //73,A,29

    let left_value = values.get(0).unwrap();
    let right_value = values.get(1).unwrap();

    let before_int: u32 = left_value
        .parse()
        .map_err(|_| SequenceRuleParseError::LeftValueFailParse)?;
    let after_int: u32 = right_value
        .parse()
        .map_err(|_| SequenceRuleParseError::RightValueFailParse)?;

    Ok(SequenceRule {
        before_int: before_int,
        after_int: after_int,
    })
}

#[test]
fn test_parse_rule() {
    let str = "47|53".to_string();
    let result = parse_rule(str);
    assert_eq!(
        result,
        Ok(SequenceRule {
            before_int: 47,
            after_int: 53
        })
    )
}
