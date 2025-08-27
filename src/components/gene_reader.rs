use bevy::prelude::*;
use gene_traits::{dna, rna};

#[derive(Component)]
pub struct GeneParser {}

#[derive(Component)]
pub struct AttachedGenome {
    pub genome_entity: Entity,
    pub start_index: usize,
}

// A component to store the full genome on a separate entity
#[derive(Component)]
pub struct Genome {
    pub sequence: Vec<dna::Nucleotide>,
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
    query: Query<&AttachedGenome, With<GeneParser>>,
    genomes: Query<&Genome>,
    mut commands: Commands,
) {
    for attached in query.iter() {
        let Ok(genome) = genomes.get(attached.genome_entity) else {
            continue;
        };
        let start = attached.start_index.min(genome.sequence.len());
        let slice = &genome.sequence[start..];
        if let Some(rna_strand) = parse_gene(slice) {
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
    use super::{AttachedGenome, GeneParser, Genome, RnaStrand, parse_attached_genome};
    use bevy::{
        app::{App, Update},
        ecs::world::World,
    };
    use gene_traits::{dna, rna};

    use crate::components::gene_reader::parse_gene;

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

    #[test]
    fn integration_spawns_rna_from_attached_genome_with_start_index() {
        // Full genome with two promoters; we start parsing at index 0.
        let genome = vec![
            dna::Nucleotide::A,
            dna::Nucleotide::T,
            dna::Nucleotide::T,
            dna::Nucleotide::A, // promoter
            dna::Nucleotide::G,
            dna::Nucleotide::A,
            dna::Nucleotide::A,
            dna::Nucleotide::T,
            dna::Nucleotide::T,
            dna::Nucleotide::A, // next promoter -> end
        ];

        let expected = RnaStrand(vec![
            rna::Nucleotide::U,
            rna::Nucleotide::U,
            rna::Nucleotide::A,
            rna::Nucleotide::G,
            rna::Nucleotide::A,
        ]);

        let mut app = App::new();
        app.add_systems(Update, parse_attached_genome);

        // Spawn the genome holder entity
        let genome_entity = app.world_mut().spawn(Genome { sequence: genome }).id();

        // Spawn an entity with the parser and an attachment referencing the genome entity
        app.world_mut().spawn((
            GeneParser {},
            AttachedGenome {
                genome_entity,
                start_index: 0,
            },
        ));

        // Run the system
        app.update();

        // Verify an RnaStrand was spawned and matches expectation
        let world: &mut World = app.world_mut();
        let mut query = world.query::<&RnaStrand>();

        let strands: Vec<&RnaStrand> = query.iter(app.world()).collect();
        assert!(!strands.is_empty(), "No RnaStrand entities were spawned");

        // There may be multiple, check at least one matches expected
        assert!(
            strands.iter().any(|s| **s == expected),
            "No spawned RnaStrand matched the expected sequence"
        );
    }

    #[test]
    fn integration_respects_start_index_slice() {
        // Genome where promoter occurs later; start parsing mid-genome
        // Genome layout: [noise..., promoter, U, U, A, promoter, ...]
        let genome = vec![
            dna::Nucleotide::G,
            dna::Nucleotide::G,
            dna::Nucleotide::G,
            dna::Nucleotide::A, // noise
            dna::Nucleotide::A,
            dna::Nucleotide::T, // -> U
            dna::Nucleotide::T, // -> U
            dna::Nucleotide::A, // -> A
            dna::Nucleotide::G, // -> G
            dna::Nucleotide::A, // -> A
            dna::Nucleotide::A,
            dna::Nucleotide::T,
            dna::Nucleotide::T,
            dna::Nucleotide::A, // next promoter
        ];

        // Starting at index 4, so the first window that can form the promoter is at 5..8
        let start_index = 4;

        let expected = RnaStrand(vec![
            rna::Nucleotide::U,
            rna::Nucleotide::U,
            rna::Nucleotide::A,
            rna::Nucleotide::G,
            rna::Nucleotide::A,
        ]);

        let mut app = App::new();
        app.add_systems(Update, parse_attached_genome);

        let genome_entity = app.world_mut().spawn(Genome { sequence: genome }).id();

        app.world_mut().spawn((
            GeneParser {},
            AttachedGenome {
                genome_entity,
                start_index,
            },
        ));

        app.update();

        let world: &mut World = app.world_mut();
        let mut query = world.query::<&RnaStrand>();
        let strands: Vec<&RnaStrand> = query.iter(app.world()).collect();

        assert!(!strands.is_empty(), "No RnaStrand entities were spawned");
        assert!(
            strands.iter().any(|s| **s == expected),
            "No spawned RnaStrand matched the expected sequence for the given start index"
        );
    }
}
