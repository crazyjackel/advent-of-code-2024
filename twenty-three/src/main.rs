use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split('-').collect();
        let res = map.entry(split[0]).or_insert(Vec::new());
        res.push(split[1]);
        let res = map.entry(split[1]).or_insert(Vec::new());
        res.push(split[0]);
    }

    let mut largest_clique = find_largest_clique(&map);
    largest_clique.sort();
    println!("{:?}", largest_clique.join(","));

    // let mut largest_hashset: HashSet<&str> = HashSet::new();
    // for key in map.keys(){
    //     if largest_hashset.contains(key){
    //         continue;
    //     }
    //     let mut stack : VecDeque<(HashSet<&str>,&str)> = VecDeque::new();
    //     stack.push_back((HashSet::new(), key));
    //     while let Some(element) = stack.pop_back(){
    //         if element.0 != largest_hashset && largest_hashset.contains(element.1){
    //             continue;
    //         }
    //         let vector : HashSet<&&str> = map[element.1].iter().collect();
    //         let map_contains_all = element.0.iter().all(|item| vector.contains(&item));
    //         if map_contains_all{
    //             for ele in vector{
    //                 if element.0 != largest_hashset && largest_hashset.contains(ele){
    //                     continue;
    //                 }
    //                 let mut new_hashset = element.0.clone();
    //                 new_hashset.insert(element.1);
    //                 stack.push_back((new_hashset, ele));
    //             }
    //         }else if element.0.len() > largest_hashset.len(){
    //             largest_hashset = element.0;
    //         }
    //     }
    // }
    // let mut vec: Vec<&str> = largest_hashset.into_iter().collect();
    // vec.sort();
    // println!("{:?}", vec.join(","));
    // let mut largest_hashset: HashSet<&str> = HashSet::new();
    // for key in map.keys(){
    //     if largest_hashset.contains(key){
    //         continue;
    //     }
    //     let mut queue : VecDeque<(HashSet<&str>,&str)> = VecDeque::new();
    //     while let Some(key) = queue.pop_front(){
    //         let vector : Vec<&&str> = map[key.1].iter().collect();
    //         let map_contains_all = key.0.iter().all(|item| vector.contains(&item));
    //         if map_contains_all{
    //             for ele in vector{
    //                 let mut new_hashset = key.0.clone();
    //                 new_hashset.insert(key.1);
    //                 queue.push_front((new_hashset, ele));
    //             }
    //         }else if key.0.len() > largest_hashset.len(){
    //             largest_hashset = key.0;
    //         }
    //     }

    //     // let mut checked : HashSet<&str> = HashSet::new();
    //     // let mut queue : VecDeque<&str> = VecDeque::new();
    //     // queue.push_front(key);
    //     // while let Some(key) = queue.pop_front(){
    //     //     if checked.contains(key){
    //     //         continue;
    //     //     }
    //     //     checked.insert(key);
    //     //     for connection in &map[key]{
    //     //         queue.push_back(connection);
    //     //     }
    //     // }
    //     // if checked.len() > largest_hashset.len(){
    //     //     largest_hashset = checked;
    //     // }
    // }
    // let mut vec: Vec<&str> = largest_hashset.into_iter().collect();
    // vec.sort();
    // println!("{:?}", vec.join(","));
    // let mut count = 0;
    // let mut checked: HashSet<&str> = HashSet::new();
    // for key in map.keys() {
    //     if key.starts_with('t') {
    //         for key_2 in &map[key] {
    //             if !checked.contains(key_2) {
    //                 for key_3 in &map[key_2] {
    //                     if !checked.contains(key_3){
    //                         for key_4 in &map[key_3]{
    //                             if key_4 == key{
    //                                 count += 1;
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         checked.insert(key);
    //     }
    // }
    // println!("{}", count / 2);
}


#[test]
fn test_2() {
    let contents = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split('-').collect();
        let res = map.entry(split[0]).or_insert(Vec::new());
        res.push(split[1]);
        let res = map.entry(split[1]).or_insert(Vec::new());
        res.push(split[0]);
    }

    // let mut largest_hashset: HashSet<&str> = HashSet::new();
    // for key in map.keys(){
    //     if largest_hashset.contains(key){
    //         continue;
    //     }
    //     let mut queue : VecDeque<(HashSet<&str>,&str)> = VecDeque::new();
    //     queue.push_front((HashSet::new(), key));
    //     while let Some(element) = queue.pop_front(){
    //         if element.0 != largest_hashset && largest_hashset.contains(element.1){
    //             continue;
    //         }
    //         let vector : HashSet<&&str> = map[element.1].iter().collect();
    //         let map_contains_all = element.0.iter().all(|item| vector.contains(&item));
    //         if map_contains_all{
    //             for ele in vector{
    //                 if element.0 != largest_hashset && largest_hashset.contains(ele){
    //                     continue;
    //                 }
    //                 let mut new_hashset = element.0.clone();
    //                 new_hashset.insert(element.1);
    //                 queue.push_front((new_hashset, ele));
    //             }
    //         }else if element.0.len() > largest_hashset.len(){
    //             largest_hashset = element.0;
    //         }
    //     }
    // }
    let mut largest_clique = find_largest_clique(&map);
    largest_clique.sort();
    println!("{:?}", largest_clique.join(","));
}


fn get_longest_path<'a>(
    starting_point: &str,
    node: &'a str,
    preceding_path: &Vec<&'a str>,
    visited: &mut HashSet<&'a str>,
    map: &HashMap<&str, Vec<&'a str>>
) -> Vec<&'a str> {
    if visited.contains(node) {
        if starting_point == node {
            return preceding_path.to_vec();
        }
        return Vec::new();
    }
    visited.insert(node);
    let mut longest_path : Vec<&str> = Vec::new();
    for child in &map[node]{
        let mut new_path = preceding_path.clone();
        new_path.push(node);
        let value = get_longest_path(starting_point, child, &new_path, visited, map);
        if value.len() > longest_path.len(){
            longest_path = value;
        }
    }
    visited.remove(node);
    longest_path
}

#[test]
fn test() {
    let contents = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split('-').collect();
        let res = map.entry(split[0]).or_insert(Vec::new());
        res.push(split[1]);
        let res = map.entry(split[1]).or_insert(Vec::new());
        res.push(split[0]);
    }

    let mut count = 0;
    let mut checked: HashSet<&str> = HashSet::new();
    for key in map.keys() {
        if key.starts_with('t') {
            for key_2 in &map[key] {
                if !checked.contains(key_2) {
                    for key_3 in &map[key_2] {
                        if !checked.contains(key_3) {
                            for key_4 in &map[key_3] {
                                if !checked.contains(key_4) {
                                    if key_4 == key {
                                        println!("{}", key);
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            checked.insert(key);
        }
    }
    println!("{}", count / 2);
}

fn find_largest_clique_recursive<'a>(
    graph: &HashMap<&str, Vec<&str>>,
    current_clique: Vec<&'a str>,
    candidates: Vec<&'a str>,
    largest_clique: &mut Vec<&'a str>,
) {
    //If current_clique is larger than largest clique, then it is largest_clique
    if current_clique.len() > largest_clique.len() {
        *largest_clique = current_clique.clone();
    }

    //Candidates are elements that could be added to the clique
    for (i, &candidate) in candidates.iter().enumerate() {
        let mut new_clique = current_clique.clone();
        new_clique.push(candidate);

        //Get a set of candidates that connect back to our first candidate
        let new_candidates: Vec<&str> = candidates[i + 1..]
            .iter()
            .filter(|&&v| graph[&candidate].contains(&v))
            .cloned()
            .collect();
        find_largest_clique_recursive(graph, new_clique, new_candidates, largest_clique);
    }
}

fn find_largest_clique<'a>(graph: &'a HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut largest_clique = Vec::new();
    let vertices: Vec<&str> = graph.keys().cloned().collect();

    find_largest_clique_recursive(graph, Vec::new(), vertices, &mut largest_clique);

    largest_clique
}