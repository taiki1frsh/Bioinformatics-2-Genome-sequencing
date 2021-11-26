use std::env;

// require the command line argument in this fn
pub fn arange_for_eulerian_cycle() -> (Vec<i32>, Vec<Vec<i32>>, i32) {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    let mut lines = std::io::BufRead::lines(reader).into_iter();

    let mut node_elements: Vec<String> = lines
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
            let vec_str: Vec<i32> = node_elements[i]
                .split(' ')
                .map(|x| x.parse().unwrap_or(0) as i32)
                .collect();           
            num_element += vec_str.len();
            out_vec.insert(count2, vec_str);
            count2 += 1;
            
        }
    }
    (in_vec, out_vec, num_element as i32)
}