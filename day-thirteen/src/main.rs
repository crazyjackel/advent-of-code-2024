use std::{fs::File, io::Read, ops::Index};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tokens: i64 = get_formulas(&contents)
        .iter_mut()
        .map(|x|{
            x.0.result += 10000000000000i64;
            x.1.result += 10000000000000i64;
            x
        })
        .filter_map(|(x, y)| {
            // if !x.has_solution(){
            //     return None;
            // }

            // if !y.has_solution(){
            //     return None;
            // }

            let new_form = LinearDiophantineFormula {
                result: x.result - y.result,
                a_factor: x.a_factor - y.a_factor,
                b_factor: x.b_factor - y.b_factor,
            };

            let solution = new_form.get_solutions();
            if solution.is_none() {
                return None;
            }
            let result = solution.unwrap();
            let first = x.result - (x.a_factor * result.a0) - (x.b_factor * result.b0);
            let second = x.a_factor * result.a_growth + x.b_factor * result.b_growth;

            if first % second != 0 {
                return None;
            }
            let k = first / second;

            let num_a = result.a0 + k * result.a_growth;
            let num_b = result.b0 + k * result.b_growth;

            Some(3 * num_a + num_b)
        })
        .fold(0, |mut acc, ele| {
            acc += ele; //if ele.is_some() { ele.unwrap() } else { 0 };
            acc
        });
    println!("Fewest Tokens {}", tokens);
}

fn get_formulas(contents: &String) -> Vec<(LinearDiophantineFormula, LinearDiophantineFormula)> {
    let strings: Vec<&str> = contents.split_whitespace().collect();
    let formulas: Vec<(LinearDiophantineFormula, LinearDiophantineFormula)> = strings
        .chunks(11)
        .map(|x| {
            let id2 = x[2];
            let id6 = x[6];
            let id9 = x[9];

            let id3 = x[3];
            let id7 = x[7];
            let id10: &str = x[10];

            (
                LinearDiophantineFormula {
                    result: id9[id9.find('=').unwrap() + 1..id9.find(',').unwrap()]
                        .parse::<i64>()
                        .unwrap(),
                    a_factor: id2[id2.find('+').unwrap() + 1..id2.find(',').unwrap()]
                        .parse()
                        .unwrap(),
                    b_factor: id6[id6.find('+').unwrap() + 1..id6.find(',').unwrap()]
                        .parse()
                        .unwrap(),
                },
                LinearDiophantineFormula {
                    result: id10[id10.find('=').unwrap() + 1..id10.len()]
                        .parse::<i64>()
                        .unwrap(),
                    a_factor: id3[id3.find('+').unwrap() + 1..id3.len()].parse().unwrap(),
                    b_factor: id7[id7.find('+').unwrap() + 1..id7.len()].parse().unwrap(),
                },
            )
        })
        .collect();
    formulas
}

#[test]
fn test() {
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens: i64 = get_formulas(&contents)
        .iter()
        .filter_map(|(x, y)| {
            // if !x.has_solution(){
            //     return None;
            // }

            // if !y.has_solution(){
            //     return None;
            // }

            let new_form = LinearDiophantineFormula {
                result: x.result - y.result,
                a_factor: x.a_factor - y.a_factor,
                b_factor: x.b_factor - y.b_factor,
            };

            let solution = new_form.get_solutions();
            if solution.is_none() {
                return None;
            }
            let result = solution.unwrap();
            let first = x.result - (x.a_factor * result.a0) - (x.b_factor * result.b0);
            let second = x.a_factor * result.a_growth + x.b_factor * result.b_growth;

            if first % second != 0 {
                return None;
            }
            let k = first / second;

            let num_a = result.a0 + k * result.a_growth;
            let num_b = result.b0 + k * result.b_growth;

            Some(3 * num_a + num_b)
        })
        .fold(0, |mut acc, ele| {
            acc += ele; //if ele.is_some() { ele.unwrap() } else { 0 };
            acc
        });

    println!("Fewest Tokens {}", tokens);
}

#[test]
fn test_solve() {
    let formula = LinearDiophantineFormula {
        result: 8400,
        a_factor: 94,
        b_factor: 22,
    };
    let solution = formula.get_solutions();
    assert!(solution.is_some());
}

#[test]
fn test_gcd_basic() {
    assert_eq!(gcd(48, 18), 6); // GCD of 48 and 18 is 6
    assert_eq!(gcd(101, 103), 1); // Coprime numbers
    assert_eq!(gcd(0, 5), 5); // GCD with zero
    assert_eq!(gcd(5, 0), 5); // GCD with zero
    assert_eq!(gcd(0, 0), 0); // Edge case: both zero
}

// trait ValidCombos<T> {
//     fn get_valid_combos(&self) -> Vec<(T, T)>;
// }

// impl ValidCombos<u64> for (LinearDiophantineFormula, LinearDiophantineFormula) {
//     fn get_valid_combos(&self) -> Vec<(u64, u64)> {
//         let mut combos: Vec<(u64, u64)> = Vec::new();
//         let (formula1, formula2) = self;
//         let bigger_formula = if formula1.result >= formula2.result {
//             formula1
//         } else {
//             formula2
//         };
//         let smaller_formula = if formula1.result >= formula2.result {
//             formula2
//         } else {
//             formula1
//         };

//         if bigger_formula.a_factor > smaller_formula.a_factor{
//             let new_result = (bigger_formula.result - smaller_formula.result);
//             let new_a_factor = (bigger_formula.a_factor - smaller_formula.a_factor);
//             let a_min = new_result / new_a_factor;
//             for a in a_min..100{
//                 if (bigger_formula.result - (bigger_formula.a_factor * a)) % bigger_formula.b_factor != 0{
//                     continue;
//                 }
//                 if (smaller_formula.result - (smaller_formula.a_factor * a)) % smaller_formula.b_factor != 0{
//                     continue;
//                 }
//                 let b1 = (bigger_formula.result - (bigger_formula.a_factor * a)) / bigger_formula.b_factor;
//                 let b2 =(smaller_formula.result - (smaller_formula.a_factor * a)) / smaller_formula.b_factor;
//                 if b1 == b2{
//                     combos.push((a,b1));
//                 }
//             }
//         }else if bigger_formula.b_factor > smaller_formula.b_factor{
//             let b_max = u64::min(
//                 bigger_formula.result / bigger_formula.b_factor,
//                 smaller_formula.result / smaller_formula.b_factor,
//             );
//             let b_min = (bigger_formula.result - smaller_formula.result) / (bigger_formula.b_factor - smaller_formula.b_factor);
//             for b in b_min..b_max{
//                 if (bigger_formula.result - (bigger_formula.b_factor * b)) % bigger_formula.a_factor != 0{
//                     continue;
//                 }
//                 if (smaller_formula.result - (smaller_formula.b_factor * b)) % smaller_formula.a_factor != 0{
//                     continue;
//                 }
//                 let a1 = (bigger_formula.result - (bigger_formula.b_factor * b)) / bigger_formula.a_factor;
//                 let a2 =(smaller_formula.result - (smaller_formula.b_factor * b)) / smaller_formula.a_factor;
//                 if a1 == a2{
//                     combos.push((a1,b));
//                 }
//             }
//         }else{
//             let a_max = u64::min(
//                 bigger_formula.result / bigger_formula.a_factor,
//                 smaller_formula.result / smaller_formula.a_factor,
//             );
//             let a_min = 0;
//             for a in a_min..a_max{
//                 if (bigger_formula.result - (bigger_formula.a_factor * a)) % bigger_formula.b_factor != 0{
//                     continue;
//                 }
//                 if (smaller_formula.result - (smaller_formula.a_factor * a)) % smaller_formula.b_factor != 0{
//                     continue;
//                 }
//                 let b1 = (bigger_formula.result - (bigger_formula.a_factor * a)) / bigger_formula.b_factor;
//                 let b2 =(smaller_formula.result - (smaller_formula.a_factor * a)) / smaller_formula.b_factor;
//                 if b1 == b2{
//                     combos.push((a,b1));
//                 }
//             }
//         }
//         combos
//     }
// }

#[derive(Debug, PartialEq)]
struct LinearDiophantineFormula {
    result: i64,
    a_factor: i64,
    b_factor: i64,
}

#[derive(Debug, PartialEq)]
struct LinearDiophantineSolution {
    a0: i64,
    b0: i64,
    a_growth: i64,
    b_growth: i64,
}

impl LinearDiophantineFormula {
    fn has_solution(&self) -> bool {
        (self.result % gcd(self.a_factor, self.b_factor) == 0)
    }

    fn get_solutions(&self) -> Option<LinearDiophantineSolution> {
        if !self.has_solution() {
            return None;
        }

        let gcd = gcd(self.a_factor, self.b_factor);

        let scaled_result = self.result as i64 / gcd;
        let a_factor = self.a_factor as i64 / gcd;
        let b_factor = self.b_factor as i64 / gcd;

        let (_, x, y) = bezout(a_factor, b_factor);

        // Scale the particular solution to the actual result
        let a0 = x * scaled_result;
        let b0 = y * scaled_result;

        // General solution growth terms
        let a_growth = b_factor; // Growth for A
        let b_growth = -a_factor; // Growth for B

        Some(LinearDiophantineSolution {
            a0,
            b0,
            a_growth,
            b_growth,
        })
    }
}

//Returns Bezout's Identity: aX+bY=gcd(a,b). Return Format (gcd(a,b), a, b)
fn bezout(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        // Base case: gcd(a, 0) = a, so x = 1, y = 0
        (a, 1, 0)
    } else {
        // Recursive case
        let (gcd, x1, y1) = bezout(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}
