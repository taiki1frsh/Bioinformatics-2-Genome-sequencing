use std::collections::HashMap;

use crate::w3::complement::{rev_complements, just_complement};

use super::amino_data::get_amino_table;

pub fn generate_all_rna_seqs_from_peptide(peptide: &str) -> Vec<String> {
    let mut whole_rna_set: Vec<String> = Vec::new();
    let mut amino_table = HashMap::new();
    amino_table = get_amino_table(amino_table);
    let mut rev_amino_table = HashMap::new();
    for i in amino_table.keys() {
        if rev_amino_table.contains_key(amino_table[i]) {
            continue;
        }

        let mut vec = Vec::new();
        for k in amino_table.keys() {
            if amino_table[k] == amino_table[i] {
                vec.insert(vec.len(), k);
            }
        }
        rev_amino_table.insert(amino_table[i], vec);
    }

    //make ingredients to construct whole seq
    let mut string_vec = Vec::new();
    for i in 0..peptide.len() {
        string_vec.insert(i, rev_amino_table[&peptide[i..i+1]].clone());       
    }

    for i in 0..string_vec.len() {
        let rna_set_num = whole_rna_set.len();
        if rna_set_num == 0 {
            for k in string_vec[0].clone() {
                whole_rna_set.insert(whole_rna_set.len(), k.to_string())
            }
        }
        for k in string_vec[i].iter() {
            for j in 0..rna_set_num {
                whole_rna_set.push(whole_rna_set[j].clone() + **k);
            }
        }
        for k in 0..rna_set_num {
            whole_rna_set.remove(0);
        }
    }
    whole_rna_set
}

pub fn detect_rna_seq(rna: &str, amino_set: &Vec<String>) -> Vec<String> {
    let mut rna_set = Vec::new();
    let amino_len = amino_set.last().unwrap().len();
    for i in 0..rna.len() - amino_len {
        let string = &rna[i..i + amino_len];
        for k in amino_set.iter() {
            if k == string {
                rna_set.push(string.to_string());
            }
        }
    }
    rna_set
}

#[test]
fn test_detect_seq_in_rna() {
    let dna = "ATGGCCATGGCCCCCAGAACTGAGATCAATAGTACCCGTATTAACGGGTGA";
    let rna = dna.replace("T", "U");
    let peptide = "MA";
    let whole_rna_set = generate_all_rna_seqs_from_peptide(peptide);
    println!("{whole_rna_set:?} \n {}", whole_rna_set.len());
    //to detect forwading seq
    let amino_set = detect_rna_seq(&rna, &whole_rna_set);
    let mut dna_set = Vec::with_capacity(amino_set.len());
    for i in 0..amino_set.len() {
        dna_set.push(amino_set[i].replace("U", "T"));
    }
    println!("{dna_set:?}");

    let rna_comp = rev_complements(&rna);
    println!("{rna_comp}");
    let rna_comp_set = detect_rna_seq(&rna_comp, &whole_rna_set);
    println!("{rna_comp_set:?}");
}