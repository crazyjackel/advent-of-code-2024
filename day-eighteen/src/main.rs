use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{Empty, Read},
};

use grid::Grid;

#[derive(Debug, PartialEq)]
enum GridItem {
    Wall,
    Empty,
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem::Empty
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut grid = Grid::new(71, 71);
    let mut distance_map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut previous_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut priority_queue: BinaryHeap<((usize, usize), u32)> = BinaryHeap::new();
    let mut path_hash : Option<HashSet<(usize,usize)>> = None;
    let updates = get_updates(contents);

    for update in updates{
        if let Some(index) = grid.get_mut(update.0, update.1){
            *index = GridItem::Wall
        }

        if let Some(ref hash) = path_hash{
            if !hash.contains(&update){
                continue;
            }
        }
        print_grid(&grid);

        priority_queue.clear();
        distance_map.clear();
        previous_map.clear();
        priority_queue.push(((0, 0), 0));
        distance_map.insert((0, 0), 0);
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
                    previous_map.insert(neighbor.clone(), node.clone());
                    priority_queue.push((neighbor.clone(), new_cost));
                }
            }
        }

        let mut index = (70,70);
        let mut new_path_hash: HashSet<(usize,usize)> = HashSet::new();
        while index != (0,0){
            if let Some(new_index) = previous_map.get(&index){
                new_path_hash.insert(*new_index);
                index = *new_index;
            }else{
                println!("{},{}", update.1,update.0);
                break;
            }
        }
        path_hash = Some(new_path_hash);
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

}

#[test]
fn test(){
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut grid = Grid::new(7, 7);
    let mut distance_map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut previous_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut priority_queue: BinaryHeap<((usize, usize), u32)> = BinaryHeap::new();
    let mut path_hash : Option<HashSet<(usize,usize)>> = None;
    let updates = get_updates(contents);

    for update in updates{
        if let Some(index) = grid.get_mut(update.0, update.1){
            *index = GridItem::Wall
        }

        if let Some(ref hash) = path_hash{
            if !hash.contains(&update){
                continue;
            }
        }
        print_grid(&grid);

        priority_queue.clear();
        distance_map.clear();
        previous_map.clear();
        priority_queue.push(((0, 0), 0));
        distance_map.insert((0, 0), 0);
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
                    previous_map.insert(neighbor.clone(), node.clone());
                    priority_queue.push((neighbor.clone(), new_cost));
                }
            }
        }

        let mut index = (6,6);
        let mut new_path_hash: HashSet<(usize,usize)> = HashSet::new();
        while index != (0,0){
            if let Some(new_index) = previous_map.get(&index){
                new_path_hash.insert(*new_index);
                index = *new_index;
            }else{
                println!("{},{}", update.1,update.0);
                break;
            }
        }
        path_hash = Some(new_path_hash);
    }
}

fn get_updates(contents: String) -> Vec<(usize,usize)>{
    let mut vec = Vec::new();
    for instruction in contents.split_whitespace(){
        let elements :Vec<&str> = instruction.split(',').collect();
        let col: usize = elements[0].parse().unwrap();
        let row: usize = elements[1].parse().unwrap();
        vec.push((row, col));
    }
    vec
}

// fn process_grid(grid: &mut Grid<GridItem>, contents: String){
//     let mut count = 0;
//     for instruction in contents.split_whitespace(){
//         if count >= 1024{
//             break;
//         }
//         let elements :Vec<&str> = instruction.split(',').collect();
//         let col: u32 = elements[0].parse().unwrap();
//         let row: u32 = elements[1].parse().unwrap();

//         if let Some(index) = grid.get_mut(row, col){
//             *index = GridItem::Wall;
//         }
//         count += 1;
//     }
// }

fn get_neighbors(tile: (usize, usize), grid: &Grid<GridItem>) -> Vec<((usize, usize), u32)> {
    let mut push_vec = Vec::new();

    if tile.0 < grid.rows() {
        let index = (tile.0 + 1, tile.1 + 0);
        match grid.get(index.0, index.1){
            Some(GridItem::Empty) => {
                push_vec.push((index, 1));
            }
            _ => {}
        }
    }

    if tile.1 < grid.cols() {
        let index = (tile.0 + 0, tile.1 + 1);
        match grid.get(index.0, index.1){
            Some(GridItem::Empty) => {
                push_vec.push((index, 1));
            }
            _ => {}
        }
    }

    if tile.0 > 0 {
        let index = (tile.0 - 1, tile.1 + 0);
        match grid.get(index.0, index.1){
            Some(GridItem::Empty) => {
                push_vec.push((index, 1));
            }
            _ => {}
        }
    }

    if tile.1 > 0 {
        let index = (tile.0 + 0, tile.1 - 1);
        match grid.get(index.0, index.1){
            Some(GridItem::Empty) => {
                push_vec.push((index, 1));
            }
            _ => {}
        }
    }
    push_vec
}

// fn create_grid(contents: String) -> Grid<GridItem> {
//     let lines: Vec<&str> = contents.lines().into_iter().collect();
//     let row_count = lines.len();
//     let vec: Vec<GridItem> = lines
//         .iter()
//         .flat_map(|x| {
//             x.chars()
//                 .filter_map(|y| match y {
//                     '#' => Some(GridItem::Wall),
//                     'S' => Some(GridItem::Start),
//                     '.' => Some(GridItem::Empty),
//                     'E' => Some(GridItem::End),
//                     _ => None,
//                 })
//                 .collect::<Vec<GridItem>>()
//         })
//         .collect();
//     let column_count = vec.len() / row_count;
//     let grid = Grid::from_vec(vec, column_count);
//     grid
// }

fn print_grid(grid: &Grid<GridItem>) {
    for row in grid.iter_rows() {
        let row_string: String = row
            .map(|x| match x {
                GridItem::Wall => '#',
                GridItem::Empty => ' ',
            })
            .collect();
        println!("{}", row_string);
    }
}
