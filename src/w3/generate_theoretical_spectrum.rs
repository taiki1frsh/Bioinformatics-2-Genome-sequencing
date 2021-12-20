use core::num;
use std::collections::HashMap;

use super::amino_data::get_amino_mass_table;

pub fn generate_theoretical_spectrum(amino_seq: &str) -> Vec<usize> {
    let mut subpeptides_masses = vec![0];
    let mut mass_table = HashMap::new();
    mass_table = get_amino_mass_table(mass_table);

    let subpep_seqs = generate_subpeptides_seqs(amino_seq);
    for i in 0..subpep_seqs.len() {
        let mut sum_mass = 0;
        for k in subpep_seqs[i].chars() {
            sum_mass += mass_table[&k];
        }
        subpeptides_masses.push(sum_mass);
    }

    subpeptides_masses
}

pub fn generate_subpeptides_seqs(amino_seq: &str) -> Vec<String> {
    let mut subpep_seqs = Vec::new();
    let amino_len = amino_seq.len();
    let revised_amino_seq = amino_seq.to_string() + &amino_seq;
    for i in 1..amino_len {
        for k in 0..amino_len {
            if k == amino_len {continue; }
            subpep_seqs.push(revised_amino_seq[k..k + i].to_string());
        }
    }
    subpep_seqs.push(amino_seq.chars().rev().collect());
    subpep_seqs
}

fn calculate_num_of_subpeptides_from_aminoLength(amino_length: usize) -> usize {
    let num_of_subpeptides = amino_length * (amino_length - 1);
    num_of_subpeptides
}

#[test]
fn test_generate_theoretical_spectrum() {
    //test for calculate subpep num and generate subpep seqs
    let sample_amino_len = 31315;
    let num_of_subpeps = calculate_num_of_subpeptides_from_aminoLength(
        sample_amino_len
    );
    println!("{num_of_subpeps}");

    let sample_amino_seq = "LEQN";
    let subpep_seqs = generate_subpeptides_seqs(sample_amino_seq);
    println!("{subpep_seqs:?}");

    //test for generate theoretical spectrum
    let mut subpep_masses = generate_theoretical_spectrum(sample_amino_seq);
    subpep_masses.sort_unstable();

    let mut ans = vec![0, 113, 114, 128, 129, 227, 242, 242, 257, 355, 356, 370, 371, 484];
    ans.sort_unstable();
    assert_eq!(subpep_masses, ans);
    println!("{subpep_masses:?}");
}