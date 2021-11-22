pub fn compositon(strings: &str, k: usize) -> Vec<&str> {
    let mut comp_kmers = Vec::new();
    for i in 0..=strings.len() - k {
        let kmer = &strings[i..i + k];
        comp_kmers.insert(i, kmer);
    }
    comp_kmers
}

#[test]
fn test_composition() {
    let k = 5;
    let sample_strings = "CAATCCAAC";
    let comp_kmers = compositon(sample_strings, k);

    let ans = vec!["CAATC", "AATCC", "ATCCA", "TCCAA", "CCAAC"];
    assert_eq!(comp_kmers, ans);
}