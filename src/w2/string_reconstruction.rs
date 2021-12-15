use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::w1::debruijn_from_kmers_copy::debruijn_from_kmers;

use super::data_structure::UnweightedAdjacencyList;


pub fn string_reconstruciton(k: usize, patterns: Vec<&str>) -> String {
    let db_map = debruijn_from_kmers(patterns.clone());
    let (index_map, rev_index) = make_index(&db_map);
    let mut tuple = vec![[0; 2]];
    for i in db_map.keys() {
        for k in &db_map[i] {
            tuple.push([index_map[i], index_map[k]]);
        }
    }
    let _trash = tuple.remove(0);
    let path_elements = UnweightedAdjacencyList::new_directed(index_map.len()*2, tuple.as_slice());

    let path = path_elements.eulerian_path().unwrap();
    let mut string = "".to_string();
    for i in 0..path.len() {
        if i == 0 {
            string += &rev_index[&path[i]][0..k - 1];
        } else {
            string += &rev_index[&path[i]][k - 2..k - 1];
        }
    }
    println!("{string}");
    string
}


fn make_index<'a>(map: &HashMap<&'a str, Vec<&'a str>>) -> (BTreeMap<&'a str, usize>, BTreeMap<usize, &'a str>) {
    let mut index_map = BTreeMap::new();
    //to avoid index number confrictioin
    let mut count = 0;
    for i in map.keys() {
        if !index_map.contains_key(i) {
            index_map.insert(i.clone(), count);
            count += 1;
        }
          
        for k in &map[i] {
            if !index_map.contains_key(k) {
                index_map.insert(k.clone(), count);
                count += 1;
            }
        }
    }
    let mut rev_index = BTreeMap::new();
    for i in index_map.keys() {
        rev_index.insert(index_map[i], i.clone());
    }

    (index_map, rev_index)
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_make_index() {
        let mut map = HashMap::new();
        map.insert("AC", vec!["CG"]);
        map.insert("GA", vec!["AT"]);
        map.insert("CG", vec!["GA"]);
        let _tree = make_index(&map);
    }

    #[test]
    fn test_string_reconstruction() {
        let pattern = "CTTA ACCA TACC GGCT GCTT TTAC";
        let patterns = pattern.split(' ').collect();
        let _string = string_reconstruciton(4, patterns);
    }
}