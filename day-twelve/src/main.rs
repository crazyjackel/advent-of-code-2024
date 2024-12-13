use std::{
    char,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

use grid::Grid;

struct Tile {
    row: usize,
    column: usize,
    has_neighbor_north: bool,
    has_neighbor_east: bool,
    has_neighbor_south: bool,
    has_neighbor_west: bool,
}

struct Region {
    character: char,
    elements: Vec<Tile>,
}

fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        let row_string: String = row.map(|x| x.to_string()).collect();
        println!("{}", row_string);
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
    let regions = get_regions(&grid);
    
    let price = get_price(&regions);
    println!("Total Price is {}", price);
}

fn get_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut check_set: HashSet<(usize, usize)> = HashSet::new();
    for (loc, ele) in grid.indexed_iter() {
        if !check_set.contains(&loc) {
            let mut elements: Vec<Tile> = Vec::new();
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back(loc);
            while let Some(location) = queue.pop_front() {
                if check_set.contains(&location) {
                    continue;
                }
                check_set.insert(location);

                let mut tile = Tile {
                    row: location.0,
                    column: location.1,
                    has_neighbor_north: false,
                    has_neighbor_east: false,
                    has_neighbor_south: false,
                    has_neighbor_west: false,
                };

                if let Some(element) = grid.get(location.0 + 1, location.1) {
                    if element == ele {
                        tile.has_neighbor_south = true;
                        queue.push_back((location.0 + 1, location.1));
                    }
                }
                if let Some(element) = grid.get(location.0, location.1 + 1) {
                    if element == ele {
                        tile.has_neighbor_east = true;
                        queue.push_back((location.0, location.1 + 1));
                    }
                }
                if location.0 >= 1 {
                    if let Some(element) = grid.get(location.0 - 1, location.1) {
                        if element == ele {
                            tile.has_neighbor_north = true;
                            queue.push_back((location.0 - 1, location.1));
                        }
                    }
                }

                if location.1 >= 1 {
                    if let Some(element) = grid.get(location.0, location.1 - 1) {
                        if element == ele {
                            tile.has_neighbor_west = true;
                            queue.push_back((location.0, location.1 - 1));
                        }
                    }
                }

                elements.push(tile);
            }
            regions.push(Region {
                character: *ele,
                elements: elements,
            });
        }
    }
    regions
}

#[test]
fn test() {
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let grid = create_grid(contents);
    let regions = get_regions(&grid);

    let price = get_price(&regions);
    println!("Total Price is {}", price);
}

fn get_price(regions: &Vec<Region>) -> u32{
    let mut price = 0;
    for region in regions {
        let mut sides = 0;
        sides += region
            .elements
            .iter()
            .filter(|x| !x.has_neighbor_south)
            .map(|x| (x.row, x))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.entry(key).or_insert_with(Vec::new).push(value);
                acc
            })
            .values()
            .map(|x| {
                let mut tile: Vec<usize> = x.iter().map(|x| x.column).collect();
                tile.sort();
                let mut count: u32 = 1;
                for i in 1..tile.len() {
                    if tile[i] != tile[i - 1] + 1 {
                        count += 1;
                    }
                }
                count
            })
            .fold(0, |mut acc, ele| {
                acc += ele;
                acc
            });

        sides += region
            .elements
            .iter()
            .filter(|x| !x.has_neighbor_north)
            .map(|x| (x.row, x))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.entry(key).or_insert_with(Vec::new).push(value);
                acc
            })
            .values()
            .map(|x| {
                let mut tile: Vec<usize> = x.iter().map(|x| x.column).collect();
                tile.sort();
                let mut count: u32 = 1;
                for i in 1..tile.len() {
                    if tile[i] != tile[i - 1] + 1 {
                        count += 1;
                    }
                }
                count
            })
            .fold(0, |mut acc, ele| {
                acc += ele;
                acc
            });
        sides += region
            .elements
            .iter()
            .filter(|x| !x.has_neighbor_east)
            .map(|x| (x.column, x))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.entry(key).or_insert_with(Vec::new).push(value);
                acc
            })
            .values()
            .map(|x| {
                let mut tile: Vec<usize> = x.iter().map(|x| x.row).collect();
                tile.sort();
                let mut count: u32 = 1;
                for i in 1..tile.len() {
                    if tile[i] != tile[i - 1] + 1 {
                        count += 1;
                    }
                }
                count
            })
            .fold(0, |mut acc, ele| {
                acc += ele;
                acc
            });
        sides += region
            .elements
            .iter()
            .filter(|x| !x.has_neighbor_west)
            .map(|x| (x.column, x))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.entry(key).or_insert_with(Vec::new).push(value);
                acc
            })
            .values()
            .map(|x| {
                let mut tile: Vec<usize> = x.iter().map(|x| x.row).collect();
                tile.sort();
                let mut count: u32 = 1;
                for i in 1..tile.len() {
                    if tile[i] != tile[i - 1] + 1 {
                        count += 1;
                    }
                }
                count
            })
            .fold(0, |mut acc, ele| {
                acc += ele;
                acc
            });
        price += region.elements.len() as u32 * sides;
    }
    price
}
