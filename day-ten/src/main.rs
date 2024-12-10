use std::{
    char,
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use grid::Grid;

fn print_grid(grid: &Grid<u8>) {
    for row in grid.iter_rows() {
        let row_string: String = row.map(|x| x.to_string()).collect();
        println!("{}", row_string);
    }
}

fn create_grid(contents: String) -> Grid<u8> {
    let lines: Vec<&str> = contents.lines().into_iter().collect();
    let row_count = lines.len();
    let vec: Vec<u8> = lines
        .iter()
        .flat_map(|x| {
            x.chars()
                .filter_map(|y| match y {
                    char => {
                        if char.is_digit(10) {
                            Some((char as u32 - '0' as u32) as u8)
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<u8>>()
        })
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
    let mut count = 0;
    let mut count2 = 0;
    for (loc, height) in grid.indexed_iter() {
        if height == &0 {
            let mut hashset: HashMap<(usize, usize), u32> = HashMap::new();
            get_trail_ends(&mut hashset, loc, &grid, 0);
            count += hashset.len() as u32;
            for i in hashset.iter(){
                count2 += i.1;
            }
        }
    }
    println!("Total Trailends {}", count);
    println!("Total Distinct Trails {}", count2);
}

fn get_trail_ends(
    trail_ends: &mut HashMap<(usize, usize), u32>,
    starting_location: (usize, usize),
    grid: &Grid<u8>,
    height: u8
) {
    let location_option = grid.get(starting_location.0, starting_location.1);
    if location_option.is_none() {
        return;
    }
    let location = location_option.unwrap();

    if location != &height{
        return;
    }

    if location == &9u8 {
        let trails = trail_ends.entry(starting_location).or_insert(0);
        *trails += 1;
        return;
    }

    get_trail_ends(
        trail_ends,
        (starting_location.0 + 1, starting_location.1),
        grid,
        height + 1
    );
    get_trail_ends(
        trail_ends,
        (starting_location.0, starting_location.1 + 1),
        grid,
        height + 1
    );

    if starting_location.0 > 0 {
        get_trail_ends(
            trail_ends,
            (starting_location.0 - 1, starting_location.1),
            grid,
            height + 1
        );
    }

    if starting_location.1 > 0 {
        get_trail_ends(
            trail_ends,
            (starting_location.0, starting_location.1 - 1),
            grid,
            height + 1
        );
    }
}

#[test]
fn test_map() {
    let contents =  "89010123
                        78121874
                        87430965
                        96549874
                        45678903
                        32019012
                        01329801
                        10456732"
        .to_string();
    let grid = create_grid(contents);
    let mut count = 0;
    let mut count2 = 0;
    for (loc, height) in grid.indexed_iter() {
        if height == &0 {
            let mut hashset: HashMap<(usize, usize), u32> = HashMap::new();
            get_trail_ends(&mut hashset, loc, &grid, 0);
            count += hashset.len() as u32;
            for i in hashset.iter(){
                count2 += i.1;
            }
        }
    }
    println!("Total Trail {}", count);
    println!("Total Distinct Trails {}", count2);
}
