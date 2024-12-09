use std::{char, collections::HashMap, fs::File, io::Read};

use grid::Grid;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
enum GridItem {
    Empty,
    AntiNode,
    Attenna(char),
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem::Empty
    }
}

impl GridItem {
    fn to_char(&self) -> char {
        match self {
            GridItem::Empty => ' ',
            GridItem::Attenna(char) => *char,
            GridItem::AntiNode => '#',
        }
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
                    char => Some(GridItem::Attenna(char)),
                })
                .collect::<Vec<GridItem>>()
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
    let frequency_grid = get_frequency_grid(&grid);

    print_grid(&grid);
    print_grid(&frequency_grid);
    println!("Unique Locations {}", frequency_grid.iter().filter(|x| x == &&GridItem::AntiNode).count());
}

fn get_frequency_grid(grid: &Grid<GridItem>) -> Grid<GridItem> {
    let mut attenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut frequency_grid: Grid<GridItem> = Grid::new(grid.size().0, grid.size().1);

    for grid_cell in grid.indexed_iter() {
        match grid_cell.1 {
            GridItem::Attenna(char) => {
                let attenas = attenas.entry(*char).or_insert(Vec::new());
                for existing_attena in attenas.iter() {
                    update_frequency_grid(existing_attena, grid_cell.0, &mut frequency_grid);
                }
                attenas.push(grid_cell.0);
            }
            _ => {}
        }
    }
    frequency_grid
}

fn update_frequency_grid(existing_attena: &(usize, usize), grid_cell: (usize,usize), frequency_grid: &mut Grid<GridItem>) {
    for i in 0..frequency_grid.size().0{
        let antinode_1_coordinate: (i32, i32) = (
            existing_attena.0 as i32
                + (i as i32 + 1) * (grid_cell.0 as i32 - existing_attena.0 as i32),
            existing_attena.1 as i32
                + (i as i32 + 1) * (grid_cell.1 as i32 - existing_attena.1 as i32),
        );
        if let Some(loc) = frequency_grid.get_mut(antinode_1_coordinate.0, antinode_1_coordinate.1){
            *loc = GridItem::AntiNode;
        }
        else{
            break;
        }
    }     
    for i in 0..frequency_grid.size().0{
        let antinode_2_coordinate: (i32, i32) = (
            grid_cell.0 as i32
                + (i as i32 + 1) * (existing_attena.0 as i32 - grid_cell.0 as i32),
                grid_cell.1 as i32
                + (i as i32 + 1) * (existing_attena.1 as i32 - grid_cell.1 as i32),
        );
        if let Some(loc) = frequency_grid.get_mut(antinode_2_coordinate.0, antinode_2_coordinate.1){
            *loc = GridItem::AntiNode;
        }else{
            break;
        }
    }
}

#[test]
fn test_input(){

    let mut file = File::open("test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let grid = create_grid(contents);
    let frequency_grid = get_frequency_grid(&grid);

    print_grid(&grid);
    print_grid(&frequency_grid);
    println!("Unique Locations {}", frequency_grid.iter().filter(|x| x == &&GridItem::AntiNode).count());
}