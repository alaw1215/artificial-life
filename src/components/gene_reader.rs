use bevy::prelude::*;
use gene_traits::{dna, rna};

#[derive(Component)]
pub struct GeneParser {}

#[derive(Component)]
pub struct AttachedGenome<'a> {
    genome: &'a [dna::Nucleotide],
}

const TATA_BOXES: [[dna::Nucleotide; 4]; 4] = [
    [
        dna::Nucleotide::A,
        dna::Nucleotide::T,
        dna::Nucleotide::T,
        dna::Nucleotide::A,
    ],
    [
        dna::Nucleotide::T,
        dna::Nucleotide::A,
        dna::Nucleotide::T,
        dna::Nucleotide::A,
    ],
    [
        dna::Nucleotide::T,
        dna::Nucleotide::A,
        dna::Nucleotide::A,
        dna::Nucleotide::T,
    ],
    [
        dna::Nucleotide::T,
        dna::Nucleotide::T,
        dna::Nucleotide::T,
        dna::Nucleotide::A,
    ],
];

#[derive(Component, Deref, PartialEq, Eq, Debug)]
pub struct RnaStrand(Vec<rna::Nucleotide>);

pub fn parse_attached_genome(
    query: Query<&AttachedGenome<'static>, With<GeneParser>>,
    mut commands: Commands,
) {
    for genome in query.iter() {
        if let Some(rna_strand) = parse_gene(genome.genome) {
            commands.spawn(rna_strand);
        }
    }
}

fn parse_gene(genome: &[dna::Nucleotide]) -> Option<RnaStrand> {
    let gene_window: &mut [dna::Nucleotide; 4] = &mut [dna::Nucleotide::A; 4];

    let mut rna_strand: Vec<rna::Nucleotide> = Vec::new();

    let mut found_start = false;
    let mut found_end = false;

    for i in 3..genome.len() {
        gene_window[0] = genome[i - 3];
        gene_window[1] = genome[i - 2];
        gene_window[2] = genome[i - 1];
        gene_window[3] = genome[i];
        if TATA_BOXES.contains(gene_window) {
            if found_start && found_end {
                if rna_strand.len() > 0 {
                    return Some(RnaStrand(rna_strand));
                }
                return None;
            }
            found_start = true;
            continue;
        }
        found_end = true;

        rna_strand.push(gene_window[0].into());
    }

    if rna_strand.len() > 0 {
        return Some(RnaStrand(rna_strand));
    }
    None
}

#[cfg(test)]
mod test {
    use gene_traits::{dna, rna};

    use crate::components::gene_reader::{RnaStrand, parse_gene};

    #[test]
    fn get_rna_strand() {
        let genome = [
            dna::Nucleotide::A,
            dna::Nucleotide::T,
            dna::Nucleotide::T,
            dna::Nucleotide::A,
            dna::Nucleotide::G,
            dna::Nucleotide::A,
            dna::Nucleotide::A,
            dna::Nucleotide::T,
            dna::Nucleotide::T,
            dna::Nucleotide::A,
        ];

        let expected = RnaStrand(vec![
            rna::Nucleotide::U,
            rna::Nucleotide::U,
            rna::Nucleotide::A,
            rna::Nucleotide::G,
            rna::Nucleotide::A,
        ]);

        let result = parse_gene(&genome).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn get_rna_strand_end() {
        let genome = [
            dna::Nucleotide::A,
            dna::Nucleotide::T,
            dna::Nucleotide::T,
            dna::Nucleotide::A,
            dna::Nucleotide::G,
            dna::Nucleotide::A,
        ];

        let expected = RnaStrand(vec![rna::Nucleotide::U, rna::Nucleotide::U]);

        let result = parse_gene(&genome).unwrap();

        assert_eq!(result, expected);
    }
}
