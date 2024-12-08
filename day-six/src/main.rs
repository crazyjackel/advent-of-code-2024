use std::{fs::File, io::Read};

use enumflags2::{bitflags, BitFlags};
use grid::Grid;

// How many loops can you make

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
enum GridItem {
    Obstacle,
    Empty,
    Start,
    Visited(BitFlags<Direction>),
}

impl GridItem {
    fn to_char(&self) -> char {
        match self {
            GridItem::Obstacle => '#',
            GridItem::Empty => ' ',
            GridItem::Start => '^',
            GridItem::Visited(direction) => {
                if *direction == BitFlags::default() | Direction::Up {
                    return 'N';
                }
                if *direction == BitFlags::default() | Direction::Down {
                    return 'S';
                }
                if *direction == BitFlags::default() | Direction::Left {
                    return 'W';
                }
                if *direction == BitFlags::default() | Direction::Right {
                    return 'E';
                }
                if *direction == Direction::Right | Direction::Left{
                    return '-';
                }
                if *direction == Direction::Up | Direction::Down {
                    return '|';
                }
                'X'
            }
        }
    }

    fn visit_item(&mut self, direction: &Direction) {
        if let GridItem::Visited(visit_direction) = self {
            *visit_direction |= *direction;
        } else {
            let flag: BitFlags<Direction> = BitFlags::default() | direction.clone();
            *self = GridItem::Visited(flag);
        }
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

impl Direction {
    fn to_i32_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    location: (i32, i32),
}

struct Simulation {
    grid: Grid<GridItem>,
    original_grid: Grid<GridItem>,
    guard_state: Guard,
    num_loops: u32,
    num_steps: u32,
    count_loops: bool,
}

#[derive(Debug, PartialEq)]
enum SimulationCreationError {
    StartingLocationNotFound,
}

#[derive(Debug, PartialEq)]
enum SimulationResult {
    Success,
    InfiniteLoop,
    Timeout,
}

#[derive(Debug, PartialEq)]
enum SimulationStepResult {
    Escaped,
    DiscoveredLoop,
    Updated,
    Trapped,
}

impl Simulation {
    fn new_full(
        grid: Grid<GridItem>,
        guard_state: Guard,
        count_loops: bool,
    ) -> Result<Self, SimulationCreationError> {
        Ok(Simulation {
            grid: grid.clone(),
            original_grid: grid,
            guard_state: guard_state,
            num_loops: 0,
            num_steps: 0,
            count_loops,
        })
    }

    fn new_with_count_loops(
        grid: Grid<GridItem>,
        count_loops: bool,
    ) -> Result<Self, SimulationCreationError> {
        let starting_location = grid.indexed_iter().find(|x| x.1 == &GridItem::Start);
        if starting_location.is_none() {
            return Err(SimulationCreationError::StartingLocationNotFound);
        }
        let location = starting_location.unwrap();
        let guard_state = Guard {
            direction: Direction::Up,
            location: (location.0 .0 as i32, location.0 .1 as i32),
        };
        Simulation::new_full(grid, guard_state, count_loops)
    }

    fn new(grid: Grid<GridItem>) -> Result<Self, SimulationCreationError> {
        Simulation::new_with_count_loops(grid, false)
    }

    fn run_simulation(&mut self) -> SimulationResult {
        let size = self.grid.size();
        let check_bound = size.0 * size.1;
        for i in 0..check_bound + 1 {
            let step_result = self.run_simulation_step();
            if matches!(
                step_result,
                SimulationStepResult::Trapped | SimulationStepResult::Escaped
            ) {
                break;
            }

            if step_result == SimulationStepResult::DiscoveredLoop {
                return SimulationResult::InfiniteLoop;
            }

            if i == check_bound {
                return SimulationResult::Timeout;
            }
        }
        self.num_steps = self
            .grid
            .iter()
            .filter(|x| {
                if let GridItem::Visited(_) = x {
                    true
                } else {
                    false
                }
            })
            .count() as u32;
        SimulationResult::Success
    }

    fn run_simulation_step(&mut self) -> SimulationStepResult {
        let initial_direction = self.guard_state.direction.clone();
        for _ in 0..4 {
            let direction_tuple = self.guard_state.direction.to_i32_tuple();
            let next_coordinate = (
                self.guard_state.location.0 + direction_tuple.0,
                self.guard_state.location.1 + direction_tuple.1,
            );
            let grid_item = self.grid.get_mut(next_coordinate.0, next_coordinate.1);
            if grid_item.is_none() {
                let mark = self
                    .grid
                    .get_mut(self.guard_state.location.0, self.guard_state.location.1)
                    .unwrap();
                mark.visit_item(&self.guard_state.direction);
                return SimulationStepResult::Escaped;
            }
            let item = grid_item.unwrap();
            if item == &GridItem::Obstacle {
                self.guard_state.direction = self.guard_state.direction.turn_right();
                continue;
            }


            if let GridItem::Visited(flags) = item {
                if flags.contains(self.guard_state.direction) {
                    let mark = self
                        .grid
                        .get_mut(self.guard_state.location.0, self.guard_state.location.1)
                        .unwrap();
                    mark.visit_item(&initial_direction);
                    return SimulationStepResult::DiscoveredLoop;
                }
            }else if self.count_loops {

                let mut new_grid = self.original_grid.clone();
                let access = new_grid
                    .get_mut(next_coordinate.0, next_coordinate.1)
                    .unwrap();
                *access = GridItem::Obstacle;
                let new_simulation = Simulation::new(new_grid).ok();
                if let Some(mut new_sim) = new_simulation {
                    self.num_loops += if new_sim.run_simulation() == SimulationResult::InfiniteLoop
                    {
                        // print_grid(&new_sim.grid);
                        // println!("");
                        1
                    } else {
                        0
                    };
                }
            }

            let mark = self
                .grid
                .get_mut(self.guard_state.location.0, self.guard_state.location.1)
                .unwrap();
            mark.visit_item(&initial_direction);
            self.guard_state.location = next_coordinate;
            return SimulationStepResult::Updated;
        }
        SimulationStepResult::Trapped
    }
}

fn print_grid(grid: &Grid<GridItem>) {
    for row in grid.iter_rows() {
        let row_string: String = row.map(|x| x.to_char()).collect();
        println!("{}", row_string);
    }
}

fn create_grid(contents: String) -> Grid<GridItem> {
    let lines: Vec<&str> = contents.lines().into_iter().collect();
    let row_count = lines.len();
    let vec: Vec<GridItem> = lines
        .iter()
        .flat_map(|x| {
            x.chars()
                .filter_map(|y| match y {
                    '.' => Some(GridItem::Empty),
                    '^' => Some(GridItem::Start),
                    '#' => Some(GridItem::Obstacle),
                    _ => None,
                })
                .collect::<Vec<GridItem>>()
        })
        .collect();
    let column_count = vec.len() / row_count;
    let grid = Grid::from_vec(vec, column_count);
    grid
}

// fn check_for_loop(
//     grid: &Grid<GridItem>,
//     starting_location: (i32, i32),
//     direction: Direction,
// ) -> bool {
//     let mut count = 1;
//     let mut if_next_tile_is_obstacle = false;
//     let direction_tuple = direction.to_i32_tuple();
//     loop {
//         let check_coordinate = (
//             starting_location.0 + count * direction_tuple.0,
//             starting_location.1 + count * direction_tuple.1,
//         );
//         let item_option = grid.get(check_coordinate.0, check_coordinate.1);
//         if item_option.is_none() {
//             return false;
//         }
//         let item = item_option.unwrap();

//         match item {
//             GridItem::Obstacle => {
//                 if if_next_tile_is_obstacle {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//             GridItem::VisitedLeftRight => {
//                 if direction == Direction::Up || direction == Direction::Down {
//                     if_next_tile_is_obstacle = true
//                 }
//             }
//             GridItem::VisitedUpDown => {
//                 if direction == Direction::Left || direction == Direction::Right {
//                     if_next_tile_is_obstacle = true
//                 }
//             }
//             GridItem::VisitedAny => return true,
//             _ => {}
//         }

//         count += 1;
//     }
// }

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let grid = create_grid(contents);
    let mut simulation = Simulation::new_with_count_loops(grid, true).unwrap();
    let result = simulation.run_simulation();
    print_grid(&simulation.grid);
    println!("Count is {}", simulation.num_steps);
    println!("Num Loops is {}", simulation.num_loops);
}

#[test]
fn test_example() {
    let mut file = File::open("test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let grid = create_grid(contents);
    let mut simulation = Simulation::new_with_count_loops(grid, true).unwrap();
    let result = simulation.run_simulation();
    assert_eq!(SimulationResult::Success, result);
    print_grid(&simulation.grid);
    assert_eq!(41, simulation.num_steps);
    assert_eq!(6, simulation.num_loops);
}

#[test]
fn test_infinite() {
    let mut file = File::open("test_infinite.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let grid = create_grid(contents);
    let mut simulation = Simulation::new(grid).unwrap();
    let result = simulation.run_simulation();
    assert_eq!(SimulationResult::InfiniteLoop, result);
    print_grid(&simulation.grid);
}
