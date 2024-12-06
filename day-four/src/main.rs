use std::{fs::File, io::Read};

use grid::Grid;

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum XMAS {
    X,
    M,
    A,
    S,
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let map: Vec<Vec<XMAS>> = contents
        .lines()
        .into_iter()
        .map(|x| {
            x.chars()
                .filter_map(|y| match y {
                    'X' => Some(XMAS::X),
                    'M' => Some(XMAS::M),
                    'A' => Some(XMAS::A),
                    'S' => Some(XMAS::S),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let column_size = map.first().unwrap().len();
    let flat_map: Vec<XMAS> = map.into_iter().flatten().collect();
    let grid = Grid::from_vec(flat_map, column_size);

    process_grid_2(grid);
}

fn process_grid_2(grid: Grid<XMAS>) {
    let mut count = 0;
    for ((row, col), i) in grid.indexed_iter() {
        if i == &XMAS::A {
            if is_x_mas(&grid, row, col) == Ok(true) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn process_grid_1(grid: Grid<XMAS>) {
    let mut count = 0;
    let check_directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
    ];

    for ((row, col), i) in grid.indexed_iter() {
        if i == &XMAS::X {
            for direction in check_directions.iter() {
                if is_xmas(&grid, *direction, row, col) == Ok(true) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn is_x_mas<I, E>(grid: &Grid<XMAS>, starting_row: I, starting_col: I) -> Result<bool, String>
where
    I: TryInto<i32, Error = E>,
    E: std::error::Error,
{
    let row = starting_row.try_into().map_err(|x: E| format!("{}", x))?;
    let col = starting_col.try_into().map_err(|x: E| format!("{}", x))?;

    let middle = grid.get(row, col);
    let top_left = grid.get(row - 1, col - 1);
    let bottom_right = grid.get(row + 1, col + 1);

    let top_right = grid.get(row - 1, col + 1);
    let bottom_left = grid.get(row + 1, col - 1);

    if middle != Some(&XMAS::A) {
        return Err("Invalid Middle".to_string());
    }

    // Either top-left is M and bottom-right is S or top-left is S and bottom-right is S, else Err.
    if !((top_left == Some(&XMAS::M) && bottom_right == Some(&XMAS::S))
        || (top_left == Some(&XMAS::S) && bottom_right == Some(&XMAS::M)))
    {
        return Err("Top-Left to Bottom-Right Diagonal do not spell MAS".to_string());
    }

    if !((top_right == Some(&XMAS::M) && bottom_left == Some(&XMAS::S))
        || (top_right == Some(&XMAS::S) && bottom_left == Some(&XMAS::M)))
    {
        return Err("Top-Right to Bottom-Left Diagonal do not spell MAS".to_string());
    }

    Ok(true)
}

fn is_xmas<I, E>(
    grid: &Grid<XMAS>,
    vector: (i32, i32),
    starting_row: I,
    starting_col: I,
) -> Result<bool, String>
where
    I: TryInto<i32, Error = E>,
    E: std::error::Error,
{
    let row = starting_row.try_into().map_err(|x: E| format!("{}", x))?;
    let col = starting_col.try_into().map_err(|x: E| format!("{}", x))?;
    Ok(matches_xmas(&grid, (row, col), XMAS::X)
        && matches_xmas(&grid, (row + vector.0, col + vector.1), XMAS::M)
        && matches_xmas(&grid, (row + 2 * vector.0, col + 2 * vector.1), XMAS::A)
        && matches_xmas(&grid, (row + 3 * vector.0, col + 3 * vector.1), XMAS::S))
}

fn matches_xmas(grid: &Grid<XMAS>, loc: (i32, i32), xmas: XMAS) -> bool {
    grid.get(loc.0, loc.1) == Some(&xmas)
}

#[test]
fn xmas_invalidate(){
    let grid = Grid::from_vec(vec![XMAS::X,XMAS::X,XMAS::S,XMAS::X,XMAS::A,XMAS::X,XMAS::M,XMAS::X,XMAS::S], 3);
    println!("{:?}", grid);

    assert!(is_x_mas(&grid, 1, 1).ok() == None);
}


#[test]
fn xmas_validate(){
    let grid = Grid::from_vec(vec![XMAS::M,XMAS::X,XMAS::S,XMAS::X,XMAS::A,XMAS::X,XMAS::M,XMAS::X,XMAS::S], 3);
    println!("{:?}", grid);

    assert!(is_x_mas(&grid, 1, 1).ok() == Some(true));
}