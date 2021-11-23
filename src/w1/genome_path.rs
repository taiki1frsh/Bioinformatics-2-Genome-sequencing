pub fn recon_from_genome_path(collection: &Vec<String>) -> String {
    let mut genome = collection[0].to_owned();
    let len = genome.len();
    for i in 1..collection.len() {
        genome += &collection[i].chars().nth(len - 1).unwrap().to_string();
    }
    genome
}

#[test]
fn test_recon_from_genome_path() {
    let collection = vec!["ACCGA".to_string(), "CCGAA".to_string(), "CGAAG".to_string(), "GAAGC".to_string(), "AAGCT".to_string()];
    let genome = recon_from_genome_path(&collection);

    let ans = "ACCGAAGCT";
    assert_eq!(&genome, ans);
}