use std::{fs::File, io::Read};

use grid::Grid;

#[derive(Debug, PartialEq, Clone, Copy)]
enum GridItem {
    Player,
    Wall,
    Empty,
    LeftBox,
    RightBox,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
    Up,
    Down,
}

impl Instruction {
    fn move_index(
        &self,
        loc: (usize, usize),
        board_size: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Instruction::Left => {
                if loc.1 > 0 {
                    Some((loc.0, loc.1 - 1))
                } else {
                    None
                }
            }
            Instruction::Right => {
                if board_size.1 > loc.1 {
                    Some((loc.0, loc.1 + 1))
                } else {
                    None
                }
            }
            Instruction::Up => {
                if loc.0 > 0 {
                    Some((loc.0 - 1, loc.1))
                } else {
                    None
                }
            }
            Instruction::Down => {
                if board_size.0 > loc.0 {
                    Some((loc.0 + 1, loc.1))
                } else {
                    None
                }
            }
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result: Vec<&str> = contents.split("\n\n").collect();

    let mut grid = create_grid(result[0].to_string());
    let instructions = create_instruction_list(result[1].to_string());
    process_grid(&mut grid, &instructions);

    let count = grid.indexed_iter().fold(0, |mut acc, (loc, ele)| {
        match ele {
            GridItem::LeftBox => acc += 100 * loc.0 + loc.1,
            _ => {}
        }
        acc
    });

    println!("Count is {}", count);
}

#[test]
fn test() {
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result: Vec<&str> = contents.split("\n\n").collect();

    let mut grid = create_grid(result[0].to_string());
    let instructions = create_instruction_list(result[1].to_string());
    //print_grid(&grid);
    process_grid(&mut grid, &instructions);
    //print_grid(&grid);

    let count = grid.indexed_iter().fold(0, |mut acc, (loc, ele)| {
        match ele {
            GridItem::LeftBox => acc += 100 * loc.0 + loc.1,
            _ => {}
        }
        acc
    });

    println!("Count is {}", count);
}

fn process_grid(grid: &mut Grid<GridItem>, instructions: &Vec<Instruction>) {
    let bounds = (grid.rows(), grid.cols());
    let player = grid.indexed_iter().find(|x| x.1 == &GridItem::Player);
    if player.is_none() {
        return;
    }
    let (mut player_index, _) = player.unwrap();

    for instruction in instructions {
        //println!("{:?}", instruction);
        if let Some(new_index) = instruction.move_index(player_index, bounds) {
            match grid.get(new_index.0, new_index.1) {
                Some(GridItem::Empty) => {
                    let player = grid.get_mut(player_index.0, player_index.1).unwrap();
                    *player = GridItem::Empty;
                    let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                    *empty = GridItem::Player;
                    player_index = new_index;
                }
                Some(GridItem::RightBox) => {
                    if let Some(new_left_index) = Instruction::Left.move_index(new_index, bounds) {
                        if push_box_2(new_left_index, new_index, instruction, grid, true) {
                            let player = grid.get_mut(player_index.0, player_index.1).unwrap();
                            *player = GridItem::Empty;
                            let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                            *empty = GridItem::Player;
                            player_index = new_index;
                        }
                    }
                }
                Some(GridItem::LeftBox) => {
                    if let Some(new_right_index) = Instruction::Right.move_index(new_index, bounds)
                    {
                        if push_box_2(new_index, new_right_index, instruction, grid, true) {
                            let player = grid.get_mut(player_index.0, player_index.1).unwrap();
                            *player = GridItem::Empty;
                            let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                            *empty = GridItem::Player;
                            player_index = new_index;
                        }
                    }
                }
                Some(GridItem::Player) => {}
                Some(GridItem::Wall) => {}
                None => {}
            }
        }
        //print_grid(&grid);
    }
}

fn push_box_2(
    left_index: (usize, usize),
    right_index: (usize, usize),
    instruction: &Instruction,
    grid: &mut Grid<GridItem>,
    push_box: bool,
) -> bool {
    //Detect Can't Move
    let bounds = (grid.rows(), grid.cols());
    match instruction {
        Instruction::Left => {
            if let Some(new_index) = instruction.move_index(left_index, bounds) {
                match grid.get(new_index.0, new_index.1) {
                    Some(GridItem::Empty) => {
                        if push_box {
                            let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                            *empty = GridItem::LeftBox;
                            let left_index = grid.get_mut(left_index.0, left_index.1).unwrap();
                            *left_index = GridItem::RightBox;
                            let right_index = grid.get_mut(right_index.0, right_index.1).unwrap();
                            *right_index = GridItem::Empty;
                        }
                        return true;
                    }
                    Some(GridItem::RightBox) => {
                        if let Some(new_index_2) = instruction.move_index(new_index, bounds) {
                            if push_box_2(new_index_2, new_index, instruction, grid, push_box) {
                                if push_box {
                                    let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                                    *empty = GridItem::LeftBox;
                                    let left_index =
                                        grid.get_mut(left_index.0, left_index.1).unwrap();
                                    *left_index = GridItem::RightBox;
                                    let right_index =
                                        grid.get_mut(right_index.0, right_index.1).unwrap();
                                    *right_index = GridItem::Empty;
                                }
                                return true;
                            }
                        }
                        return false;
                    }
                    Some(_) => return false,
                    None => return false,
                }
            } else {
                return false;
            }
        }
        Instruction::Right => {
            if let Some(new_index) = instruction.move_index(right_index, bounds) {
                match grid.get(new_index.0, new_index.1) {
                    Some(GridItem::Empty) => {
                        if push_box {
                            let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                            *empty = GridItem::RightBox;
                            let left_index = grid.get_mut(left_index.0, left_index.1).unwrap();
                            *left_index = GridItem::Empty;
                            let right_index = grid.get_mut(right_index.0, right_index.1).unwrap();
                            *right_index = GridItem::LeftBox;
                        }
                        return true;
                    }
                    Some(GridItem::LeftBox) => {
                        if let Some(new_index_2) = instruction.move_index(new_index, bounds) {
                            if push_box_2(new_index, new_index_2, instruction, grid, push_box) {
                                if push_box {
                                    let empty = grid.get_mut(new_index.0, new_index.1).unwrap();
                                    *empty = GridItem::RightBox;
                                    let left_index =
                                        grid.get_mut(left_index.0, left_index.1).unwrap();
                                    *left_index = GridItem::Empty;
                                    let right_index =
                                        grid.get_mut(right_index.0, right_index.1).unwrap();
                                    *right_index = GridItem::LeftBox;
                                }
                                return true;
                            }
                        }
                        return false;
                    }
                    Some(_) => return false,
                    None => return false,
                }
            } else {
                return false;
            }
        }
        Instruction::Up => {
            if let Some(new_left_index) = instruction.move_index(left_index, bounds) {
                if let Some(new_right_index) = instruction.move_index(right_index, bounds) {
                    let left_grid_item = *grid.get(new_left_index.0, new_left_index.1).unwrap();
                    let right_grid_item = *grid.get(new_right_index.0, new_right_index.1).unwrap();
                    let can_push_box_left = match left_grid_item {
                        GridItem::Wall => false,
                        GridItem::RightBox => {
                            let mut value = false;
                            if let Some(new_lefter_index) =
                                Instruction::Left.move_index(new_left_index, bounds)
                            {
                                if push_box_2(
                                    new_lefter_index,
                                    new_left_index,
                                    instruction,
                                    grid,
                                    false,
                                ) {
                                    value = true;
                                }
                            }
                            value
                        }
                        GridItem::LeftBox => {
                            let mut value = false;
                            if push_box_2(new_left_index, new_right_index, instruction, grid, false)
                            {
                                value = true;
                            }
                            value
                        }
                        GridItem::Empty => true,
                        _ => false,
                    };
                    let can_push_box_right = match right_grid_item {
                        GridItem::Wall => false,
                        GridItem::LeftBox => {
                            let mut value = false;
                            if let Some(new_righter_index) =
                                Instruction::Right.move_index(new_right_index, bounds)
                            {
                                if push_box_2(
                                    new_right_index,
                                    new_righter_index,
                                    instruction,
                                    grid,
                                    false,
                                ) {
                                    value = true;
                                }
                            }
                            value
                        }
                        GridItem::RightBox => {
                            let mut value = false;
                            if push_box_2(new_left_index, new_right_index, instruction, grid, false)
                            {
                                value = true;
                            }
                            value
                        }
                        GridItem::Empty => true,
                        _ => false,
                    };

                    if can_push_box_left && can_push_box_right && push_box {
                        if left_grid_item == GridItem::LeftBox
                            && right_grid_item == GridItem::RightBox
                        {
                            push_box_2(new_left_index, new_right_index, instruction, grid, true);
                        } else {
                            if left_grid_item == GridItem::RightBox {
                                push_box_2(
                                    Instruction::Left
                                        .move_index(new_left_index, bounds)
                                        .unwrap(),
                                    new_left_index,
                                    instruction,
                                    grid,
                                    true,
                                );
                            }
                            if right_grid_item == GridItem::LeftBox {
                                push_box_2(
                                    new_right_index,
                                    Instruction::Right
                                        .move_index(new_right_index, bounds)
                                        .unwrap(),
                                    instruction,
                                    grid,
                                    true,
                                );
                            }
                        }
                        let left_empty = grid.get_mut(new_left_index.0, new_left_index.1).unwrap();
                        *left_empty = GridItem::LeftBox;
                        let left_index = grid.get_mut(left_index.0, left_index.1).unwrap();
                        *left_index = GridItem::Empty;
                        let right_empty =
                            grid.get_mut(new_right_index.0, new_right_index.1).unwrap();
                        *right_empty = GridItem::RightBox;
                        let right_index = grid.get_mut(right_index.0, right_index.1).unwrap();
                        *right_index = GridItem::Empty;
                    }

                    return can_push_box_left && can_push_box_right;
                }
            }
            return false;
        }
        Instruction::Down => {
            if let Some(new_left_index) = instruction.move_index(left_index, bounds) {
                if let Some(new_right_index) = instruction.move_index(right_index, bounds) {
                    let left_grid_item = *grid.get(new_left_index.0, new_left_index.1).unwrap();
                    let right_grid_item = *grid.get(new_right_index.0, new_right_index.1).unwrap();
                    let can_push_box_left = match left_grid_item {
                        GridItem::Wall => false,
                        GridItem::RightBox => {
                            let mut value = false;
                            if let Some(new_lefter_index) =
                                Instruction::Left.move_index(new_left_index, bounds)
                            {
                                if push_box_2(
                                    new_lefter_index,
                                    new_left_index,
                                    instruction,
                                    grid,
                                    false,
                                ) {
                                    value = true;
                                }
                            }
                            value
                        }
                        GridItem::LeftBox => {
                            let mut value = false;
                            if push_box_2(new_left_index, new_right_index, instruction, grid, false)
                            {
                                value = true;
                            }
                            value
                        }
                        GridItem::Empty => true,
                        _ => false,
                    };
                    let can_push_box_right = match right_grid_item {
                        GridItem::Wall => false,
                        GridItem::LeftBox => {
                            let mut value = false;
                            if let Some(new_righter_index) =
                                Instruction::Right.move_index(new_right_index, bounds)
                            {
                                if push_box_2(
                                    new_right_index,
                                    new_righter_index,
                                    instruction,
                                    grid,
                                    false,
                                ) {
                                    value = true;
                                }
                            }
                            value
                        }
                        GridItem::RightBox => {
                            let mut value = false;
                            if push_box_2(new_left_index, new_right_index, instruction, grid, false)
                            {
                                value = true;
                            }
                            value
                        }
                        GridItem::Empty => true,
                        _ => false,
                    };

                    if can_push_box_left && can_push_box_right && push_box {
                        if left_grid_item == GridItem::LeftBox
                            && right_grid_item == GridItem::RightBox
                        {
                            push_box_2(new_left_index, new_right_index, instruction, grid, true);
                        } else {
                            if left_grid_item == GridItem::RightBox {
                                push_box_2(
                                    Instruction::Left
                                        .move_index(new_left_index, bounds)
                                        .unwrap(),
                                    new_left_index,
                                    instruction,
                                    grid,
                                    true,
                                );
                            }
                            if right_grid_item == GridItem::LeftBox {
                                push_box_2(
                                    new_right_index,
                                    Instruction::Right
                                        .move_index(new_right_index, bounds)
                                        .unwrap(),
                                    instruction,
                                    grid,
                                    true,
                                );
                            }
                        }
                        let left_empty = grid.get_mut(new_left_index.0, new_left_index.1).unwrap();
                        *left_empty = GridItem::LeftBox;
                        let left_index = grid.get_mut(left_index.0, left_index.1).unwrap();
                        *left_index = GridItem::Empty;
                        let right_empty =
                            grid.get_mut(new_right_index.0, new_right_index.1).unwrap();
                        *right_empty = GridItem::RightBox;
                        let right_index = grid.get_mut(right_index.0, right_index.1).unwrap();
                        *right_index = GridItem::Empty;
                    }

                    return can_push_box_left && can_push_box_right;
                }
            }
            return false;
        }
    }

    //
}

fn push_box(
    instruction: &Instruction,
    new_index: (usize, usize),
    bounds: (usize, usize),
    grid: &mut Grid<GridItem>,
    player_index: &mut (usize, usize),
) {
    if let Some(mut box_check_index) = instruction.move_index(new_index, bounds) {
        let mut should_swap = false;
        while let Some(grid_item) = grid.get(box_check_index.0, box_check_index.1) {
            if grid_item == &GridItem::Wall {
                break;
            }

            if grid_item == &GridItem::Empty {
                should_swap = true;
                break;
            }

            if let Some(new_box_check_index) = instruction.move_index(box_check_index, bounds) {
                box_check_index = new_box_check_index;
            } else {
                break;
            }
        }

        if should_swap {
            let player = grid.get_mut(player_index.0, player_index.1).unwrap();
            *player = GridItem::Empty;
            let box_item = grid.get_mut(new_index.0, new_index.1).unwrap();
            *box_item = GridItem::Player;
            let empty = grid.get_mut(box_check_index.0, box_check_index.1).unwrap();
            *empty = GridItem::RightBox;
            *player_index = new_index;
        }
    }
}

fn create_instruction_list(content: String) -> Vec<Instruction> {
    content
        .chars()
        .into_iter()
        .filter_map(|x| match x {
            '<' => Some(Instruction::Left),
            '^' => Some(Instruction::Up),
            '>' => Some(Instruction::Right),
            'v' => Some(Instruction::Down),
            _ => None,
        })
        .collect()
}

fn create_grid(contents: String) -> Grid<GridItem> {
    let lines: Vec<&str> = contents.lines().into_iter().collect();
    let row_count = lines.len();
    let vec: Vec<GridItem> = lines
        .iter()
        .flat_map(|x| {
            x.chars()
                .filter_map(|y| match y {
                    '#' => Some((GridItem::Wall, GridItem::Wall)),
                    'O' => Some((GridItem::LeftBox, GridItem::RightBox)),
                    '.' => Some((GridItem::Empty, GridItem::Empty)),
                    '@' => Some((GridItem::Player, GridItem::Empty)),
                    _ => None,
                })
                .flat_map(|x| vec![x.0, x.1])
                .collect::<Vec<GridItem>>()
        })
        .collect();
    let column_count = vec.len() / row_count;
    let grid = Grid::from_vec(vec, column_count);
    grid
}

fn print_grid(grid: &Grid<GridItem>) {
    for row in grid.iter_rows() {
        let row_string: String = row
            .map(|x| match x {
                GridItem::Player => '@',
                GridItem::Wall => '#',
                GridItem::Empty => ' ',
                GridItem::LeftBox => '[',
                GridItem::RightBox => ']',
            })
            .collect();
        println!("{}", row_string);
    }
}
