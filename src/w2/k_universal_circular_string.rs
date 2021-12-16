use std::collections::{HashMap, BTreeMap};

use rand::Rng;

use crate::{w1::overlap_graph::{prefix, suffix}, w2::data_structure::UnweightedAdjacencyList};

pub fn k_universal_circular_string(k: usize) -> String {
    let mut string_set = Vec::new();
    let set_len = 2_usize.pow(k as u32);
    let mut rng = rand::thread_rng();
    let mut count = 0;

    loop {
        if string_set.len() == set_len  {
            break;
        } else {
                let mut string = String::new();
            for _i in 0..k {
                let num: i32 = rng.gen_range(0..2);
                let num_str = num.to_string();
                string.push_str(&num_str);
            }
            if !string_set.contains(&string) {
                string_set.insert(count, string);
                count += 1;
            }
        }
    }
    let string = string_reconstruciton(k, string_set);

    string
}

fn debruijn_from_kmers(set_of_kmers: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut deb_map = HashMap::new();
    let pre_deb_elements = Vec::new();
    let set_num = set_of_kmers.len();

    for i in 0..set_num {
        let mut sub_colc = Vec::new();
        let pre_kmer = prefix(&set_of_kmers[i]);
        if pre_deb_elements.contains(&pre_kmer) {
            continue;
        }

        for j in 0..set_num {
            let sub_kmer = set_of_kmers[j].clone();
            let pre_sub_kmer = prefix(&sub_kmer);
            let suf_sub_kmer = suffix(&sub_kmer);
            if pre_kmer == pre_sub_kmer {
                sub_colc.insert(sub_colc.len(), suf_sub_kmer.to_string());
            }
        }
        deb_map.insert(pre_kmer.to_string(), sub_colc.clone());
    }

    deb_map
}

fn string_reconstruciton(k: usize, patterns: Vec<String>) -> String {
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
    string
}

fn make_index(map: &HashMap<String, Vec<String>>) -> (BTreeMap<String, usize>, BTreeMap<usize, String>) {
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
mod test {
    use super::*;

    #[test]
    fn test_debruijn() {
        let set_of_kmers = vec!["GAGG".to_string(), "CAGG".to_string(), "GGGG".to_string(), "GGGA".to_string(), "CAGG".to_string(), "AGGG".to_string(), "GGAG".to_string()];
        let trydeb = debruijn_from_kmers(set_of_kmers);
        println!("{trydeb:?}");
    }

    #[test]
    fn test_string_recon() {
        let patterns = vec!["CTTA".to_string(), "ACCA".to_string(), "TACC".to_string(), "GGCT".to_string(), "GCTT".to_string(),"TTAC".to_string()];
        let string = string_reconstruciton(4, patterns);
        println!("{string}");
    }

    #[test]
    fn test_k_universal_circular_string() {

        let string = k_universal_circular_string(4);
        println!("{string}");
    }
}