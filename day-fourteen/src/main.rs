use std::{collections::HashSet, fs::File, io::Read};

use grid::Grid;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    draw_steps(&contents, 101, 103, 0, 100000);
}

#[test]
fn test_robots() {
    let contents = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    draw_steps(contents, 11, 7, 1, 100);
    //println!("Result is {}", (result.0 * result.1 * result.2 * result.3));
}

fn simulate(
    contents: &str,
    board_width: u32,
    board_height: u32,
    steps: u32,
) -> (i32, i32, i32, i32) {
    let result: (i32, i32, i32, i32) = contents
        .lines()
        .filter_map(|x| {
            let elements: Vec<&str> = x.split(' ').collect();
            if elements.len() != 2 {
                return None;
            }

            let position_x: i32 = elements[0]
                [elements[0].find('=').unwrap() + 1..elements[0].find(',').unwrap()]
                .parse()
                .unwrap();
            let position_y: i32 = elements[0]
                [elements[0].find(',').unwrap() + 1..elements[0].len()]
                .parse()
                .unwrap();
            let velocity_x: i32 = elements[1]
                [elements[1].find('=').unwrap() + 1..elements[1].find(',').unwrap()]
                .parse()
                .unwrap();
            let velocity_y: i32 = elements[1]
                [elements[1].find(',').unwrap() + 1..elements[1].len()]
                .parse()
                .unwrap();

            Some(((position_x, position_y), (velocity_x, velocity_y)))
        })
        .map(|((pos_x, pos_y), (vel_x, vel_y))| {
            (
                mod_positive(pos_x + vel_x * steps as i32, board_width as i32),
                mod_positive(pos_y + vel_y * steps as i32, board_height as i32),
            )
        })
        .fold((0, 0, 0, 0), |mut acc, item| {
            if item.0 < (board_width / 2) as i32 {
                if item.1 < (board_height / 2) as i32 {
                    acc.0 += 1;
                } else if item.1 > (board_height / 2) as i32 {
                    acc.1 += 1;
                }
            } else if item.0 > (board_width / 2) as i32 {
                if item.1 < (board_height / 2) as i32 {
                    acc.2 += 1;
                } else if item.1 > (board_height / 2) as i32 {
                    acc.3 += 1;
                }
            }
            acc
        });
    result
}

fn mod_positive(a: i32, n: i32) -> i32 {
    ((a % n) + n) % n
}

fn draw_steps(contents: &str, board_width: u32, board_height: u32, starting_step: u32, steps: u32) {
    let mut bots: Vec<((i32, i32), (i32, i32))> = contents
        .lines()
        .filter_map(|x| {
            let elements: Vec<&str> = x.split(' ').collect();
            if elements.len() != 2 {
                return None;
            }

            let position_x: i32 = elements[0]
                [elements[0].find('=').unwrap() + 1..elements[0].find(',').unwrap()]
                .parse()
                .unwrap();
            let position_y: i32 = elements[0]
                [elements[0].find(',').unwrap() + 1..elements[0].len()]
                .parse()
                .unwrap();
            let velocity_x: i32 = elements[1]
                [elements[1].find('=').unwrap() + 1..elements[1].find(',').unwrap()]
                .parse()
                .unwrap();
            let velocity_y: i32 = elements[1]
                [elements[1].find(',').unwrap() + 1..elements[1].len()]
                .parse()
                .unwrap();

            Some(((position_x, position_y), (velocity_x, velocity_y)))
        })
        .collect();

    let mut found_unique = false;
    let mut step = 0;
    while !found_unique {
        step += 1;
        bots = bots
            .iter()
            .map(|((pos_x, pos_y), (vel_x, vel_y))| {
                (
                    (
                        mod_positive(pos_x + vel_x, board_width as i32),
                        mod_positive(pos_y + vel_y, board_height as i32),
                    ),
                    (*vel_x, *vel_y),
                )
            })
            .collect();

        found_unique = tree_test(&bots);
    }
    let grid: Grid<u32> = bots
        .iter()
        .fold(
            Grid::new(board_height as usize, board_width as usize),
            |mut grid, ele| {
                let element = grid.get_mut(ele.0.1 as usize, ele.0.0 as usize).unwrap();
                *element += 1;
                grid
            },
        );
    print_grid(&grid);
    println!("{} -------------------", step);
    println!();
    println!();
}

fn tree_test(bots: &Vec<((i32, i32), (i32, i32))>) -> bool {
    let mut map: HashSet<(i32,i32)> = HashSet::new();
    for bot in bots.iter(){
        if map.contains(&bot.0){
            return false;
        }else{
            map.insert(bot.0);
        }
    }
    true
}

fn print_grid(grid: &Grid<u32>) {
    for row in grid.iter_rows() {
        let row_string: String = row.map(|x| if x == &0 { ' ' } else { '#' }).collect();
        println!("{}", row_string);
    }
}
