use std::{
    borrow::Cow,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::Read,
    rc::Rc,
};

#[derive(Clone, Debug, PartialOrd)]
enum Operation {
    Const(u8, String),
    AND(Box<Operation>, Box<Operation>, String),
    XOR(Box<Operation>, Box<Operation>, String),
    OR(Box<Operation>, Box<Operation>, String),
    Result(Vec<Operation>),
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Const(l0, _), Self::Const(r0, _)) => l0 == r0,
            (Self::AND(l0, l1, _), Self::AND(r0, r1, _)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            },
            (Self::XOR(l0, l1, _), Self::XOR(r0, r1, _)) => 
            {(l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)},
            (Self::OR(l0, l1, _), Self::OR(r0, r1, _)) => 
            {(l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)},
            (Self::Result(l0), Self::Result(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Operation {
    fn evaluate(&self) -> u64 {
        match self {
            Operation::Const(a, _) => (*a).into(),
            Operation::AND(operation, operation1, _) => {
                operation.evaluate() & &operation1.evaluate()
            }
            Operation::XOR(operation, operation1, _) => {
                operation.evaluate() ^ &operation1.evaluate()
            }
            Operation::OR(operation, operation1, _) => {
                operation.evaluate() | &operation1.evaluate()
            }
            Operation::Result(vec) => {
                let mut result = 0;
                for i in vec.iter().enumerate() {
                    result += 2u64.pow(i.0 as u32) * i.1.evaluate();
                }
                result
            }
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Const(_, b) => write!(f, "{}", b),
            Operation::AND(operation, operation1, str) => {
                let ord = if operation > operation1 {
                    (operation, operation1)
                } else {
                    (operation1, operation)
                };
                write!(f, "({} & {} [{}])", ord.0, ord.1, str)
            }
            Operation::XOR(operation, operation1, str) => {
                let ord = if operation > operation1 {
                    (operation, operation1)
                } else {
                    (operation1, operation)
                };
                write!(f, "({} & {} [{}])", ord.0, ord.1, str)
            }
            Operation::OR(operation, operation1, str) => {
                let ord = if operation > operation1 {
                    (operation, operation1)
                } else {
                    (operation1, operation)
                };
                write!(f, "({} & {} [{}])", ord.0, ord.1, str)
            }
            Operation::Result(vec) => {
                write!(
                    f,
                    "{{{}}}",
                    vec.iter()
                        .rev()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",\n")
                )
            }
        }
    }
}

fn main() {
    let mut file = File::open("fixed_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut construction: HashMap<&str, Operation> = HashMap::new();
    let split: Vec<&str> = contents.split("\n\n").collect();
    let lines = split[0].lines().into_iter().map(|x| {
        let result: Vec<&str> = x
            .split(|y: char| y.is_whitespace() || y == ':')
            .filter(|x| !x.is_empty())
            .collect();
        (result[0], result[1].parse::<u8>().unwrap())
    });
    for line in lines {
        construction.insert(line.0, Operation::Const(line.1, line.0.to_string()));
    }

    //println!("{}", split[1]);
    let mut queue: VecDeque<(&str, &str, &str, &str)> = split[1]
        .lines()
        .into_iter()
        .map(|x| {
            let result: Vec<&str> = x
                .split(|y: char| y.is_whitespace() || y == '-' || y == '>')
                .filter(|x| !x.is_empty())
                .collect();
            (result[0], result[1], result[2], result[3])
        })
        .collect();

    while let Some(ele) = queue.pop_front() {
        if construction.contains_key(ele.3) {
            continue;
        }

        let operation_option = get_operation(&construction, ele);
        if let Some(operation) = operation_option {
            construction.insert(ele.3, operation);
        } else {
            queue.push_back(ele);
        }
    }

    let mut x_elements: Vec<(&&str, &Operation)> = construction
        .iter()
        .filter(|x| x.0.starts_with('x'))
        .collect();
    x_elements.sort_by(|a, b| a.0.cmp(b.0));
    let x_map: Vec<Operation> = x_elements.iter().map(|x| x.1).cloned().collect();
    construction.insert(&"x", Operation::Result(x_map.clone()));
    // println!("{}", construction["x"]);
    // println!("{}", construction["x"].evaluate());

    let mut y_elements: Vec<(&&str, &Operation)> = construction
        .iter()
        .filter(|x| x.0.starts_with('y'))
        .collect();
    y_elements.sort_by(|a, b| a.0.cmp(b.0));
    let y_map: Vec<Operation> = y_elements.iter().map(|x| x.1).cloned().collect();
    construction.insert(&"y", Operation::Result(y_map.clone()));
    // println!("{}", construction["y"]);
    // println!("{}", construction["y"].evaluate());

    let mut result: Vec<Operation> = Vec::new();
    let mut carry: Option<Operation> = None;
    for i in 0..x_map.len() {
        let x = &x_map[i];
        let y = &y_map[i];
        let sum = if let Some(carry_ele) = &carry {
            Operation::XOR(
                Box::new(Operation::XOR(
                    Box::new(x.clone()),
                    Box::new(y.clone()),
                    "xxx".to_string(),
                )),
                Box::new(carry_ele.clone()),
                "xxx".to_string(),
            )
        } else {
            Operation::XOR(Box::new(x.clone()), Box::new(y.clone()), "xxx".to_string())
        };
        carry = if let Some(carry_ele) = &carry {
            Some(Operation::OR(
                Box::new(Operation::AND(
                    Box::new(x.clone()),
                    Box::new(y.clone()),
                    "xxx".to_string(),
                )),
                Box::new(Operation::AND(
                    Box::new(carry_ele.clone()),
                    Box::new(Operation::XOR(
                        Box::new(x.clone()),
                        Box::new(y.clone()),
                        "xxx".to_string(),
                    )),
                    "xxx".to_string(),
                )),
                "xxx".to_string(),
            ))
        } else {
            Some(Operation::AND(
                Box::new(x.clone()),
                Box::new(y.clone()),
                "xxx".to_string(),
            ))
        };
        result.push(sum);
    }
    if let Some(carry_ele) = carry {
        result.push(carry_ele);
    }
    // construction.insert(&"w", Operation::Result(result));
    // println!("{}", construction["w"]);
    // println!("{}", construction["w"].evaluate());

    //println!("");

    let mut z_elements: Vec<(&&str, &Operation)> = construction
        .iter()
        .filter(|x| x.0.starts_with('z'))
        .collect();
    z_elements.sort_by(|a, b| a.0.cmp(b.0));
    let z_map: Vec<Operation> = z_elements.into_iter().map(|x| x.1).cloned().collect();
    for i in 0..result.len() {
        if result[i] == z_map[i] {
            //println!("{}", result[i]);
        } else {
            println!("Correct: {}", result[i]);
            println!("Wrong: {}", z_map[i]);
        }
    }
    construction.insert(&"z", Operation::Result(z_map));
    // println!("{}", construction["z"]);
    // println!("{}", construction["z"].evaluate());

    //println!("{:?}", recipes);

    // let mut queue: VecDeque<(&str, u8)> = values.clone().into_iter().collect();
    //     let mut found_results: Vec<(&str, u8)> = Vec::new();
    //     for other_ele in &values {
    //         let other_str = *other_ele.0;

    //         if recipes.contains_key(&(ele.0, other_str)) {
    //             for recipe_result in &recipes[&(ele.0, other_str)] {
    //                 let new_u8 = recipe_result.1.evaluate(ele.1, *other_ele.1);
    //                 found_results.push((recipe_result.0, new_u8));
    //             }
    //         }
    //     }
    //     for result in found_results {
    //         queue.push_back(result);
    //         values.insert(result.0, result.1);
    //     }
    // }

    // let vec: Vec<(&str, u8)> = values
    //     .into_iter()
    //     .filter_map(|x| if x.0.starts_with('z') { Some(x) } else { None })
    //     .collect();
    // println!("{:?}", vec);
}

fn get_operation(
    construction: &HashMap<&str, Operation>,
    ele: (&str, &str, &str, &str),
) -> Option<Operation> {
    let operation_option: Option<Operation> =
        if let (Some(op0), Some(op1)) = (construction.get(&ele.0), construction.get(&ele.2)) {
            match ele.1 {
                "AND" => Some(Operation::AND(
                    Box::new(op0.clone()),
                    Box::new(op1.clone()),
                    ele.3.to_string(),
                )),
                "XOR" => Some(Operation::XOR(
                    Box::new(op0.clone()),
                    Box::new(op1.clone()),
                    ele.3.to_string(),
                )),
                "OR" => Some(Operation::OR(
                    Box::new(op0.clone()),
                    Box::new(op1.clone()),
                    ele.3.to_string(),
                )),
                _ => None,
            }
        } else {
            None
        };
    operation_option
}
