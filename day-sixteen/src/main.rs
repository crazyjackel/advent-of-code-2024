use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::Read,
};

use grid::Grid;

#[derive(Debug, PartialEq)]
enum GridItem {
    Wall,
    Start,
    End,
    Empty,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn clock_wise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn counter_clock_wise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn move_index(
        &self,
        loc: (usize, usize),
        board_size: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::West => {
                if loc.1 > 0 {
                    Some((loc.0, loc.1 - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if board_size.1 > loc.1 {
                    Some((loc.0, loc.1 + 1))
                } else {
                    None
                }
            }
            Direction::North => {
                if loc.0 > 0 {
                    Some((loc.0 - 1, loc.1))
                } else {
                    None
                }
            }
            Direction::South => {
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

    let grid = create_grid(contents);
    let mut distance_map: HashMap<((usize, usize), Direction), u32> = HashMap::new();
    let mut previous_map: HashMap<((usize, usize), Direction), Vec<((usize, usize), Direction)>> =
        HashMap::new();
    let mut priority_queue: BinaryHeap<(((usize, usize), Direction), u32)> = BinaryHeap::new();
    let mut end_index = (0, 0);

    for (loc, grid_item) in grid.indexed_iter() {
        match grid_item {
            GridItem::Wall => {}
            GridItem::Start => {
                priority_queue.push(((loc, Direction::East), 0));
                distance_map.insert((loc, Direction::East), 0);
            }
            GridItem::End => end_index = loc,
            GridItem::Empty => {}
        }
    }

    while let Some((node, cost)) = priority_queue.pop() {
        if cost > distance_map[&node] {
            continue;
        }

        for (neighbor, neighbor_cost) in get_neighbors(node, &grid) {
            let new_cost = cost + neighbor_cost;
            let distance = if distance_map.contains_key(&neighbor) {
                distance_map[&neighbor]
            } else {
                u32::MAX
            };

            if new_cost < distance {
                distance_map.insert(neighbor.clone(), new_cost);
                previous_map.insert(neighbor.clone(), vec![node.clone()]);
                priority_queue.push((neighbor.clone(), new_cost));
            } else if new_cost == distance {
                previous_map
                    .entry(neighbor.clone())
                    .or_insert(Vec::new())
                    .push(node.clone());
            }
        }
    }

    // while !priority_queue.is_empty() {
    //     let item = priority_queue.pop_min();
    //     if let Some((prior, _)) = item {
    //         let neighbors = get_neighbors(prior, &grid);
    //         let found_distance_option = distance_map.get(&prior);
    //         if found_distance_option.is_none(){
    //             continue;
    //         }
    //         let found_distance = *found_distance_option.unwrap();
    //         for (neighbor, dist) in neighbors {
    //             let prior_distance = found_distance + dist;
    //             let distance_to_neighbor = distance_map.entry(neighbor).or_insert(u32::MAX);
    //             if &prior_distance < distance_to_neighbor {
    //                 *distance_to_neighbor = prior_distance;
    //                 previous_map.insert(neighbor, prior);
    //                 if let None = priority_queue.change_priority(&neighbor, prior_distance){
    //                     println!("None");
    //                 }
    //             }
    //         }
    //     }
    // }

    let north = distance_map.get(&(end_index, Direction::North));

    println!(
        "{}",
        get_preceding_best_path(&(end_index, Direction::North), &previous_map).len()
    );
}

fn get_preceding_best_path(
    item: &((usize, usize), Direction),
    hash: &HashMap<((usize, usize), Direction), Vec<((usize, usize), Direction)>>,
) -> HashSet<(usize, usize)> {
    if !hash.contains_key(&item) {
        let mut hashset = HashSet::new();
        hashset.insert(item.0);
        return hashset;
    }
    let mut hashset = HashSet::new();
    hashset.insert(item.0);
    for preceding_item in &hash[&item] {
        for preceding_best in get_preceding_best_path(preceding_item, hash){
            hashset.insert(preceding_best);
        }
    }
    hashset
}

fn get_neighbors(
    tile: ((usize, usize), Direction),
    grid: &Grid<GridItem>,
) -> Vec<(((usize, usize), Direction), u32)> {
    let mut push_vec = Vec::new();
    push_vec.push(((tile.0, tile.1.clock_wise()), 1000));
    push_vec.push(((tile.0, tile.1.counter_clock_wise()), 1000));
    if let Some(location) = tile.1.move_index(tile.0, (grid.rows(), grid.cols())) {
        if let Some(&GridItem::Empty | &GridItem::End) = grid.get(location.0, location.1) {
            push_vec.push(((location, tile.1), 1));
        }
    }
    push_vec
}

fn create_grid(contents: String) -> Grid<GridItem> {
    let lines: Vec<&str> = contents.lines().into_iter().collect();
    let row_count = lines.len();
    let vec: Vec<GridItem> = lines
        .iter()
        .flat_map(|x| {
            x.chars()
                .filter_map(|y| match y {
                    '#' => Some(GridItem::Wall),
                    'S' => Some(GridItem::Start),
                    '.' => Some(GridItem::Empty),
                    'E' => Some(GridItem::End),
                    _ => None,
                })
                .collect::<Vec<GridItem>>()
        })
        .collect();
    let column_count = vec.len() / row_count;
    let grid = Grid::from_vec(vec, column_count);
    grid
}

// fn print_grid(grid: &Grid<GridItem>) {
//     for row in grid.iter_rows() {
//         let row_string: String = row
//             .map(|x| match x {
//                 GridItem::Player => '@',
//                 GridItem::Wall => '#',
//                 GridItem::Empty => ' ',
//                 GridItem::LeftBox => '[',
//                 GridItem::RightBox => ']',
//             })
//             .collect();
//         println!("{}", row_string);
//     }
// }
