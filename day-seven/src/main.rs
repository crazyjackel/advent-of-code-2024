use core::num;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Display,
};

#[derive(Debug, PartialEq)]
struct CalibrationFormula {
    formula_result: u64,
    formula_numbers: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
    Mult,
    Concat,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Mult => write!(f, "*"),
            Operator::Concat => write!(f, "||"),
        }
    }
}

impl Operator {
    fn evaluate(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Mult => left * right,
            Operator::Concat => left * 10u64.pow((right as f64).log10().floor() as u32 + 1) + right,
        }
    }

    fn all_variants() -> Vec<Operator> {
        vec![Operator::Concat, Operator::Add, Operator::Mult]
    }

    /// Given a Target_Value and a Left_value, tries to calculate a new target for everything to the right
    fn update_target(&self, target: u64, right_value: u64) -> Option<u64> {
        match self {
            Operator::Add => {
                if target > right_value {
                    Some(target - right_value)
                } else {
                    None
                }
            }
            Operator::Mult => {
                if target % right_value == 0 {
                    Some(target / right_value)
                } else {
                    None
                }
            }
            Operator::Concat => {
                if target > right_value
                    && (target - right_value)
                        % 10u64.pow((right_value as f64).log10().floor() as u32 + 1)
                        == 0
                {
                    Some(
                        (target - right_value)
                            / 10u64.pow((right_value as f64).log10().floor() as u32 + 1),
                    )
                } else {
                    None
                }
            }
        }
    }

    /// Given a Target_Value, evaluates whether the left_value can reach that target value through the usage of the operator and any other number.
    fn valid_left(&self, target: u64, left_value: u64) -> bool {
        match self {
            Operator::Add => target > left_value,
            Operator::Mult => target % left_value == 0,
            Operator::Concat => {
                target > left_value
                    && left_value
                        == target / 10u64.pow(target.ilog10() - left_value.ilog10() as u32)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Number(u64),
    BinaryOperation {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::BinaryOperation { op, left, right } => write!(f, "({} {} {})", left, op, right),
        }
    }
}

impl Expr {
    fn evaluate(&self) -> u64 {
        match self {
            Expr::Number(num) => *num,
            Expr::BinaryOperation { op, left, right } => {
                op.evaluate(left.evaluate(), right.evaluate())
            }
        }
    }

    /// If this Expr is to be the right side of an equation with another expression, what is a valid operator?
    /// Ex:
    /// 32 = L O 8
    /// + is valid, since 8 is less than target.
    /// * is valid, since 32 % 8 is 0
    /// || is not valid, because (32 - 8) % 10 is not 0
    /// Ex:
    /// 560 = L 0 20
    /// + is valid, since 20 is less than target.
    /// * is valid, since 560 % 20 is 0
    /// || is not valid because (560 - 20) % 100 is not 0
    fn valid_operators_left(&self, target: u64) -> Vec<Operator> {
        let number = match self {
            Expr::Number(x) => x,
            Expr::BinaryOperation { op, left, right } => {
                &op.evaluate(left.evaluate(), right.evaluate())
            }
        };

        let mut operators: Vec<Operator> = Vec::new();
        for i in Operator::all_variants() {
            if i.valid_left(target, *number) {
                operators.push(i);
            }
        }
        operators
    }

    /// Constructs all the Trees that
    fn construct_trees(numbers: &[u64], target: u64) -> Vec<Expr> {
        if numbers.is_empty() {
            return Vec::new();
        }

        if numbers.len() == 1 {
            if numbers[0] == target {
                return vec![Expr::Number(numbers[0])];
            } else {
                return Vec::new();
            }
        }

        if numbers.iter().all(|x| x > &target) {
            return Vec::new();
        }

        let mut results = Vec::new();
        for op in Operator::all_variants() {
            match op {
                // Operator::Concat => {
                //     let num_digits_plus_one = if target == 0 { 1 } else { target.ilog10() + 2 };
                //     for i in 1..numbers.len() - 1 {
                //         for j in 0..num_digits_plus_one {
                //             let left_target = target / 10u64.pow(j);
                //             let right_target = target - (left_target * 10u64.pow(j));
                //             let left_trees = Expr::construct_trees(&numbers[..i], left_target);
                //             let right_trees = Expr::construct_trees(&numbers[i..], right_target);
                //             for left in left_trees.iter() {
                //                 for right in right_trees.iter() {
                //                     let expr = Expr::BinaryOperation {
                //                         op,
                //                         left: Box::new(left.clone()),
                //                         right: Box::new(right.clone()),
                //                     };
                //                     println!("{}", expr);
                //                     results.push(expr);
                //                 }
                //             }
                //         }
                //     }
                // }
                Operator::Add | Operator::Mult | Operator::Concat => {
                    let last_number = numbers.last().unwrap();
                    if let Some(new_target) = op.update_target(target, *last_number) {
                        let left_trees =
                            Expr::construct_trees(&numbers[0..numbers.len() - 1], new_target);
                        for left in left_trees {
                            let expr = Expr::BinaryOperation {
                                op,
                                left: Box::new(left),
                                right: Box::new(Expr::Number(*last_number)),
                            };
                            println!("{}", expr);
                            results.push(expr);
                        }
                    }
                }
            }
        }

        // for i in 0..numbers.len() {
        //     let right_trees = Expr::construct_trees(&numbers[i + 1..numbers.len()], None);
        //     for right in right_trees {
        //         if let Some(new_target) = op.update_target(target.unwrap(), right.evaluate()) {
        //             let left_trees =
        //                 Expr::construct_trees(&numbers[0..i + 1], Some(new_target));
        //             for left in left_trees {
        //                 if op.valid_left(new_target, left.evaluate()) {
        //                     results.push(Expr::BinaryOperation {
        //                         op,
        //                         left: Box::new(left),
        //                         right: Box::new(right.clone()),
        //                     });
        //                 }
        //             }
        //         }
        //     }
        // }
        results
    }
}

impl CalibrationFormula {
    /// Returns a List of all the potential orderings of Operators to make the formula_result from the formula_numbers
    fn get_operators(&self) -> Vec<Vec<Operator>> {
        let len = self.formula_numbers.len();
        if len < 2 {
            return Vec::new();
        }

        if len == 2 {
            let first = self.formula_numbers.get(0).unwrap();
            let second = self.formula_numbers.get(1).unwrap();
            //Both if statements will be true if first and second are both 2

            let mut operators: Vec<Operator> = Vec::new();
            if first + second == self.formula_result {
                operators.push(Operator::Add);
            }

            if first * second == self.formula_result {
                operators.push(Operator::Mult);
            }

            return if operators.is_empty() {
                Vec::new()
            } else {
                operators.into_iter().map(|x| vec![x]).collect()
            };
        }

        let last = self.formula_numbers.last().unwrap();
        let mut operators: Vec<Vec<Operator>> = Vec::new();
        if self.formula_result % last == 0 {
            let new_calibration_formula = CalibrationFormula {
                formula_result: self.formula_result / last,
                formula_numbers: self.formula_numbers[0..len - 1].to_vec(),
            };
            let result_operators = new_calibration_formula.get_operators();
            for mut result_operator in result_operators.into_iter() {
                result_operator.push(Operator::Mult);
                operators.push(result_operator);
            }
        }

        if self.formula_result > *last {
            let new_calibration_formula = CalibrationFormula {
                formula_result: self.formula_result - last,
                formula_numbers: self.formula_numbers[0..len - 1].to_vec(),
            };
            let result_operators = new_calibration_formula.get_operators();
            for mut result_operator in result_operators.into_iter() {
                result_operator.push(Operator::Add);
                operators.push(result_operator);
            }
        }

        operators
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let formulas: Vec<CalibrationFormula> = reader
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|y| parse_calibration_formula(y).ok())
        .collect();

    let mut count: u64 = 0;
    for formula in formulas.iter() {
        // let operators = formula.get_operators();
        // if operators.len() > 1 {
        //     println!("Found One");
        // }
        let trees = Expr::construct_trees(&formula.formula_numbers, formula.formula_result);
        if !trees.is_empty() {
            count += formula.formula_result as u64;
        }
    }
    print!("Count is {}", count);
}

fn parse_calibration_formula(string: String) -> Result<CalibrationFormula, String> {
    let split: Vec<&str> = string
        .split(':')
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect();
    if split.len() != 2 {
        return Err("Bad Length".to_string());
    }
    let first_split = split.get(0).unwrap();
    let second_split = split.get(1).unwrap();

    let test_value: u64 = first_split.parse().map_err(|_| "Bad Parse")?;
    let data: Vec<u64> = second_split
        .split(' ')
        .filter(|x| !x.is_empty())
        .filter_map(|y| y.parse::<u64>().ok())
        .collect();

    Ok(CalibrationFormula {
        formula_result: test_value,
        formula_numbers: data,
    })
}

#[test]
fn test_parse_calibration_formula() {
    let str = "3267: 81 40 27".to_string();
    let result = parse_calibration_formula(str).unwrap();
    assert_eq!(3267, result.formula_result);
    assert_eq!(vec![81, 40, 27], result.formula_numbers);
}

#[test]
fn test_get_operators() {
    let str = "3267: 81 40 27".to_string();
    let result = parse_calibration_formula(str).unwrap();
    let operator_result = result.get_operators();
    assert_eq!(
        vec![
            vec![Operator::Add, Operator::Mult],
            vec![Operator::Mult, Operator::Add]
        ],
        operator_result
    );
}

#[test]
fn test_construct_trees() {
    let tree_test = vec![6, 8, 6, 15u64];
    let target = 7290u64;
    // let tree_test = vec![19,10];
    // let target = 190u64;
    let trees = Expr::construct_trees(&tree_test, target);
    for tree in trees.iter() {
        println!("{} = {}", tree, tree.evaluate());
    }
}

#[test]
fn test_output() {
    let target: u64 = 12345;
    let num_digits_plus_two: u32 = target.ilog10() + 2;
    for j in 0..num_digits_plus_two {
        let left_target = target / 10u64.pow(j);
        let right_target = target - (left_target * 10u64.pow(j));
        println!(
            "Target: {} J: {}, Left: {}, Right: {}",
            target, j, left_target, right_target
        );
    }
}
