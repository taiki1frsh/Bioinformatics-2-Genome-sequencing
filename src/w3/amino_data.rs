use std::collections::HashMap;

pub fn get_amino_table<'a>(mut amino_table: HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
    amino_table = HashMap::new();
    amino_table.insert("AAA", "K");
    amino_table.insert("AAC", "N");
    amino_table.insert("AAG", "K");
    amino_table.insert("AAU", "N");
    amino_table.insert("ACA", "T");
    amino_table.insert("ACC", "T");
    amino_table.insert("ACG", "T");
    amino_table.insert("ACU", "T");
    amino_table.insert("AGA", "R");
    amino_table.insert("AGC", "S");
    amino_table.insert("AGG", "R");
    amino_table.insert("AGU", "S");
    amino_table.insert("AUA", "I");
    amino_table.insert("AUC", "I");
    amino_table.insert("AUG", "M");
    amino_table.insert("AUU", "I");
    amino_table.insert("CAA", "Q");
    amino_table.insert("CAC", "H");
    amino_table.insert("CAG", "Q");
    amino_table.insert("CAU", "H");
    amino_table.insert("CCA", "P");
    amino_table.insert("CCC", "P");
    amino_table.insert("CCG", "P");
    amino_table.insert("CCU", "P");
    amino_table.insert("CGA", "R");
    amino_table.insert("CGC", "R");
    amino_table.insert("CGG", "R");
    amino_table.insert("CGU", "R");
    amino_table.insert("CUA", "L");
    amino_table.insert("CUC", "L");
    amino_table.insert("CUG", "L");
    amino_table.insert("CUU", "L");
    amino_table.insert("GAA", "E");
    amino_table.insert("GAC", "D");
    amino_table.insert("GAG", "E");
    amino_table.insert("GAU", "D");
    amino_table.insert("GCA", "A");
    amino_table.insert("GCC", "A");
    amino_table.insert("GCU", "A");
    amino_table.insert("GCG", "A");
    amino_table.insert("GGA", "G");
    amino_table.insert("GGU", "G");
    amino_table.insert("GGG", "G");
    amino_table.insert("GGC", "G");
    amino_table.insert("GUA", "V");
    amino_table.insert("GUC", "V");
    amino_table.insert("GUG", "V");
    amino_table.insert("GUU", "V");
    amino_table.insert("UAA", "*");
    amino_table.insert("UAC", "Y");
    amino_table.insert("UAG", "*");
    amino_table.insert("UAU", "Y");
    amino_table.insert("UCA", "S");
    amino_table.insert("UCC", "S");
    amino_table.insert("UCG", "S");
    amino_table.insert("UCU", "S");
    amino_table.insert("UGA", "*");
    amino_table.insert("UGC", "C");
    amino_table.insert("UGG", "W");
    amino_table.insert("UGU", "C");
    amino_table.insert("UUA", "L");
    amino_table.insert("UUC", "F");
    amino_table.insert("UUG", "L");
    amino_table.insert("UUU", "F");

    amino_table
}

#[test]
fn test_get_amino_table() {
    let mut amino_table = HashMap::new();
    amino_table = get_amino_table(amino_table);
    println!("{amino_table:?} - {}", amino_table.len());
}