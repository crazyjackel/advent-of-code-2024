use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

use grid::Grid;

trait Distance {
    fn manhattan_distance(&self, other: &Self) -> u32;
}

impl Distance for (usize, usize) {
    fn manhattan_distance(&self, other: &Self) -> u32 {
        ((if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        }) + (if self.1 > other.1 {
            self.1 - other.1
        } else {
            other.1 - self.1
        })) as u32
    }
}

trait Neighbors<T> {
    fn get_neighbors(&self, index: (usize, usize)) -> Vec<((usize, usize), &T)>;
}

impl<T> Neighbors<T> for Grid<T> {
    fn get_neighbors(&self, index: (usize, usize)) -> Vec<((usize, usize), &T)> {
        let mut vec: Vec<((usize, usize), &T)> = Vec::new();
        let bounds = self.size();

        //Make sure Index is actually in bounds so we can unwrap confidently
        if index.0 >= bounds.0 {
            return vec;
        }
        if index.1 >= bounds.1 {
            return vec;
        }

        if index.0 > 0 {
            let ele = self.get(index.0 - 1, index.1 + 0).unwrap();
            vec.push(((index.0 - 1, index.1 + 0), ele));
        }

        if index.1 > 0 {
            let ele = self.get(index.0 + 0, index.1 - 1).unwrap();
            vec.push(((index.0 + 0, index.1 - 1), ele));
        }

        if index.0 < bounds.0 - 1 {
            let ele = self.get(index.0 + 1, index.1 + 0).unwrap();
            vec.push(((index.0 + 1, index.1 + 0), ele));
        }
        if index.1 < bounds.1 - 1 {
            let ele = self.get(index.0 + 0, index.1 + 1).unwrap();
            vec.push(((index.0 + 0, index.1 + 1), ele));
        }
        vec
    }
}

fn create_grid(contents: String) -> Grid<char> {
    let lines: Vec<&str> = contents.lines().into_iter().collect();
    let row_count = lines.len();
    let vec: Vec<char> = lines
        .iter()
        .flat_map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let column_count = vec.len() / row_count;
    let grid = Grid::from_vec(vec, column_count);
    grid
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let grid = create_grid(contents);
    let result = grid.indexed_iter().find(|x| x.1 == &'E').unwrap();

    let mut seconds_from_end: HashMap<(usize, usize), u32> = HashMap::new();
    let mut count = 0;
    let mut seconds: VecDeque<(usize, usize)> = VecDeque::new();
    seconds.push_back(result.0);
    while !seconds.is_empty() {
        let ele = seconds.pop_front().unwrap();
        for neighbor in grid.get_neighbors(ele) {
            match neighbor.1 {
                '.' => {
                    if !seconds_from_end.contains_key(&neighbor.0) {
                        seconds.push_back(neighbor.0);
                    }
                }
                'S' => {
                    seconds_from_end.insert(neighbor.0, count + 1);
                }
                _ => {}
            }
        }
        seconds_from_end.insert(ele, count);
        count += 1;
    }

    // let mut cheats = 0;
    // for key in seconds_from_end.iter() {
    //     for neighbor in grid.get_neighbors(*key.0) {
    //         if neighbor.1 == &'#' {
    //             for neighbor_2 in grid.get_neighbors(neighbor.0) {
    //                 if neighbor_2.1 == &'.' || neighbor_2.1 == &'E' {
    //                     let ele2 = seconds_from_end[&neighbor_2.0] + 2;
    //                     if ele2 + 99 < *key.1 {
    //                         cheats += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    let mut cheats = 0;
    for bigger_index in seconds_from_end.iter() {
        for smaller_index in seconds_from_end.iter() {
            if bigger_index.1 <= smaller_index.1 {
                continue;
            }
            //It is considered a Cheat if
            // If key_distance_1 + n + 100 <= key_distance_2
            // Where n is the manhattan distance between key1 and key2 that is less than or equal to 20
            let manhattan_distance = bigger_index.0.manhattan_distance(smaller_index.0);
            if manhattan_distance > 20 {
                continue;
            }

            //If Smaller Index with the manhattan distance is less than the bigger_index it is a cheat
            if smaller_index.1 + manhattan_distance + 99 < *bigger_index.1 {
                cheats += 1;
            }
        }
    }
    println!("{}", cheats);
}
