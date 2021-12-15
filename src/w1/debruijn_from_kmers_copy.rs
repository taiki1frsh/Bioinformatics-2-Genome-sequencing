use std::collections::HashMap;

use super::overlap_graph::{prefix, suffix};

pub fn debruijn_from_kmers(set_of_kmers: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut deb_map = HashMap::new();
    let mut pre_deb_elements = Vec::new();
    //let mut suf_deb_elements = Vec::new();
    let set_num = set_of_kmers.len();

    let mut count = 0;
    for i in 0..set_num {
        let mut sub_colc = Vec::new();
        let pre_kmer = prefix(set_of_kmers[i]);
        if pre_deb_elements.contains(&pre_kmer) {
            continue;
        }

        for j in 0..set_num {
            let sub_kmer = set_of_kmers[j];
            let pre_sub_kmer = prefix(sub_kmer);
            let suf_sub_kmer = suffix(sub_kmer);
            if pre_kmer == pre_sub_kmer {
                sub_colc.insert(sub_colc.len(), suf_sub_kmer);
            }
        }
        //suf_deb_elements.insert( count, sub_colc);
        deb_map.insert(pre_kmer, sub_colc.clone());
        count += 1;
    }

    /* for i in 0..pre_deb_elements.len() {
        println!("{} -> {:?}", pre_deb_elements[i], suf_deb_elements[i]);
    } */
    //deb_map.insert(pre_deb_elements, suf_deb_elements);
    deb_map
}

#[test]
fn test_debruijn_from_kmers() {
    let string = "GAGG CAGG GGGG GGGA CAGG AGGG GGAG";
    let set_of_kmers = string.split(' ').collect();
    let deb_map = debruijn_from_kmers(set_of_kmers);
    for i in 0..deb_map.len() {
        println!("{:?}", deb_map.iter().nth(i).unwrap());
  
    }
}