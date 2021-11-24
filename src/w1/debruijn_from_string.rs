use std::collections::HashMap;

use super::overlap_graph::{prefix, suffix};

pub fn debruijn_from_string(string: &str, k: usize) -> HashMap<Vec<&str>, Vec<Vec<&str>>> {
    let mut debruijn_graph_map = HashMap::new();
    let mut debruijn_graph: Vec<Vec<&str>> = Vec::new();
    let mut origin_graph = Vec::new();
    let len = string.len() - k;

    let mut count = 0;
    //origin_graph.insert(0, suffix(&string[0..k]));
    for i in 0..=len {
        let kmer = &string[i..i + k];
        let pre_kmer = prefix(kmer);
        if !origin_graph.contains(&pre_kmer) {
            origin_graph.insert( count, pre_kmer);
        } else {
            continue;
        }

        let mut deb_colc = Vec::new();
        for j in 0..=len {
            let sub_kmer = &string[j..j + k];
            let pre_sub_kmer = prefix(sub_kmer);
            let suf_sub_kmer = suffix(sub_kmer);
            
            if pre_kmer == pre_sub_kmer {
                deb_colc.insert( deb_colc.len(), suf_sub_kmer);
            }
        }
        debruijn_graph.insert(count, deb_colc);
        count += 1;
    }
    
    for i in 0..origin_graph.len() {
        println!("{} -> {:?}", origin_graph[i], debruijn_graph[i]);
    }

    debruijn_graph_map.insert(origin_graph, debruijn_graph);
    debruijn_graph_map
}

#[test]
fn test_debruijn_from_string() {
    let k = 4;
    let string = "AAGATTCTCTAAGA";
    let deb_map = debruijn_from_string(string, k);

    println!("{:?}", deb_map);
}