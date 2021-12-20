use std::collections::HashMap;

pub fn rev_complements(genome: &str) -> String {
    let length = genome.len();
    let mut comple_table = HashMap::new();
    comple_table.insert('A', "U");
    comple_table.insert('U', "A");
    comple_table.insert('G', "C");
    comple_table.insert('C', "G");
    let rev_genome: String = genome.chars().rev().collect();
    let mut rev_comp_genome = String::new();
  
    for i in 0..length {
        rev_comp_genome += comple_table[&rev_genome.chars().nth(i).unwrap()];
    }
    
    rev_comp_genome
}

pub fn just_complement(genome: &str) -> String {
    let length = genome.len();
    let mut comple_table = HashMap::new();
    comple_table.insert('A', "U");
    comple_table.insert('U', "A");
    comple_table.insert('G', "C");
    comple_table.insert('C', "G");

    let mut comp_genome = String::new();
    for i in 0..length {
        comp_genome += comple_table[&genome.chars().nth(i).unwrap()];
    }
    comp_genome
}

#[test]
fn test_rev_complements() {
    // sample data
    let genome = "AUGCCGUA";
    let rev_comp_genome = rev_complements(genome);

    assert_eq!(rev_comp_genome, "UACGGCAU");
}
