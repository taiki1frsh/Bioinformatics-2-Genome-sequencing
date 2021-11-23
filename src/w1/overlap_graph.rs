pub fn overlap_graph(collection: &Vec<String>) {
    let colc_len = collection.len();
    let mut origin_collection = Vec::new();
    let mut overlap_graph_collection: Vec<Vec<&str>> = vec![vec![]; colc_len];
    
    let mut count = 0;
    for i in 0..colc_len {
        let kmer = collection[i].as_str();
        let suf_kmer = suffix(&kmer);
        let mut colc = vec![];
        if origin_collection.contains(&kmer) {
            continue;
        } else {
            for j in 0..colc_len {
                let pre_kmer_set = collection[j].as_str();
                if suf_kmer == prefix(&pre_kmer_set) {
                    colc.insert(colc.len(), pre_kmer_set);
                }
            }
            if colc.len() != 0 {
                origin_collection.insert(count, kmer);
                overlap_graph_collection[count] = colc;
                count += 1;
            }
        }
    }
    for i in 0..origin_collection.len() {
        println!("{} -> {:?}", origin_collection[i], overlap_graph_collection[i]);
    }
}

pub fn suffix(kmer: &str) -> &str {
    let leng = kmer.len();
    let suf_kmer = &kmer[1..leng];
    suf_kmer
}

pub fn prefix(kmer: &str) -> &str {
    let leng = kmer.len();
    let pre_kmer = &kmer[0..leng - 1];
    pre_kmer
}

mod test_overlap_graphs {
    use super::*;

    #[test]
    fn test_overlap_graph() {
        let kmers = "ATGCG GCATG CATGC AGGCA GGCAT GGCAC".to_string();
        let collection: Vec<String> = kmers.split(' ').into_iter().map(|x| x.to_owned()).collect();
        overlap_graph(&collection);
    }

    #[test]
    fn test_suffix() {
        let kmer = "ATGC";
        let suf_kmer = suffix(kmer);

        let ans = "TGC";
        assert_eq!(suf_kmer, ans);
    }

    #[test]
    fn test_prefix() {
        let kmer = "ATGC";
        let pre_kmer = prefix(kmer);

        let ans = "ATG";
        assert_eq!(pre_kmer, ans);
    }
}