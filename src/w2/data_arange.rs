use std::{collections::BTreeMap, env};

// require the command line argument in this fn
// the file format must be spefic to this format like,
// 0 <- node
// 1 2 <- edges
// continue...
pub fn arange_for_eulerian_cycle() -> (Vec<i32>, Vec<Vec<i32>>, i32) {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    let lines = std::io::BufRead::lines(reader).into_iter();

    let node_elements: Vec<String> = lines
        .map(|x| x.unwrap())
        .collect();

    let mut in_vec: Vec<i32> = Vec::new();
    let mut out_vec = Vec::new();
    let mut count1 = 0;
    let mut count2 = 0;
    let mut num_element = 0;
    for i in 0..node_elements.len() {
        if i % 2 == 0 {
            let num: i32 = node_elements[i].parse().unwrap_or(0);
            in_vec.insert(count1, num);
            count1 += 1;
        } else {
            let vec: Vec<i32> = node_elements[i]
                .split(' ')
                .map(|x| x.parse().unwrap_or(0) as i32)
                .collect();           
            num_element += vec.len();
            out_vec.insert(count2, vec);
            count2 += 1;
        }
    }
    (in_vec, out_vec, num_element as i32)
}

pub fn arange_for_eulerian_cycle_hashmap() -> (BTreeMap<i32, Vec<i32>>, usize) {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    let lines = std::io::BufRead::lines(reader).into_iter();

    let node_elements: Vec<String> = lines
        .map(|x| x.unwrap())
        .collect();

    let mut map= BTreeMap::new();
    let mut num_element = 0;
    for i in 0..node_elements.len() {
        if i % 2 == 0 {
            let num: i32 = node_elements[i].parse().unwrap_or(0);
            let vec: Vec<i32> = node_elements[i + 1]
                .split(' ')
                .map(|x| x.parse().unwrap_or(0) as i32)
                .collect();           
            num_element += vec.len();
            map.insert(num, vec);
        } else {
            continue;
        }
    }
    (map, num_element)
}