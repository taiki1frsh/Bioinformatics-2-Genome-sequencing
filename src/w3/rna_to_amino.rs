use std::collections::HashMap;

use super::amino_data::get_amino_table;

pub fn rna_to_amino_translate(rna: &str) -> String {
    let mut amino_string = String::new();
    let mut amino_table = HashMap::new();
    amino_table = get_amino_table(amino_table);

    for i in 0..( rna.len() / 3) {
        let num = i * 3;
        amino_string += amino_table[&rna[num..num + 3]];
    }
    amino_string
}

#[test]
fn test_rna_to_amino_translate() {
    let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
    let amino = &rna_to_amino_translate(rna);
    println!("{amino}");
    let ans = "MAMAPRTEINSTRING".to_string();
    assert_eq!(amino, &ans);
}