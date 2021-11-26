use std::env;

pub fn arange_for_eulerian_cycle() -> (Vec<i32>, Vec<Vec<i32>>) {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    let mut lines = std::io::BufRead::lines(reader).into_iter();

    //let pattern = lines.next().unwrap().unwrap();
    let mut sub_set_of_kmers: Vec<String> = lines
        .map(|x| x.unwrap())
        .collect();

    println!("{:?}", sub_set_of_kmers);
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2 = Vec::new();
    let mut count1 = 0;
    let mut count2 = 0;
    for i in 0..sub_set_of_kmers.len() {
        if i % 2 == 0 {
            let num: i32 = sub_set_of_kmers[i + 1].parse().unwrap_or(0);
            vec1.insert(count1, num);
            count1 += 1;
        } else {
            let vec_str: Vec<i32> = sub_set_of_kmers[i]
                .split(' ')
                .map(|x| x.parse().unwrap_or(0) as i32)
                .collect();           
            vec2.insert(count2, vec_str);
        count2 += 1;
        }
    }
    (vec1, vec2)
}