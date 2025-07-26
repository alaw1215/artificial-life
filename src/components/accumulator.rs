use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::prelude::Component;
use gene_traits::amino_acid::AminoAcid;
use gene_traits::{dna::get_header, mul, register_gene};
use hashed_type_def::HashedTypeDef;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ComponentRegister;

use crate::neurotransmitters::*;

fn accumulator_sequence_parser<T>(gene: &[AminoAcid]) -> (Accumulator<T>, usize)
where
    T: Send,
    T: Sync,
    T: Debug,
    T: 'static,
{
    let mut buildup_rate = 0;
    // The length is somehow going to have to be known.  Will have to do a pre-read until the end of the gene is encountered.
    let mut last_index = 0;
    let mut slice_window: [AminoAcid; 4] = [AminoAcid::A; 4];
    let gene_end: [AminoAcid; 4] = [AminoAcid::UNKNOWN; 4];

    let mut gene_iter = gene.iter();
    let mut consumed = gene.len();
    let mut get_last = || {
        while let Some(acid) = gene_iter.next() {
            if last_index < 3 {
                slice_window[last_index] = *acid;
            } else {
                slice_window[0] = slice_window[1];
                slice_window[1] = slice_window[2];
                slice_window[2] = slice_window[3];
                slice_window[3] = *acid;
            }

            if gene_end == slice_window {
                consumed = last_index + 1;
                last_index = last_index - 3;
                return last_index;
            }
            last_index += 1;
        }
        return last_index;
    };

    let gene_ref = &gene[0..get_last()];
    for i in 0..gene_ref.len() {
        let current_part: u32 = Into::<u8>::into(gene[i]) as u32;

        buildup_rate += current_part; // Since each amino acid can represent a value from 0-19, this seemed the simplest way to approach this
    }

    (Accumulator::new(0, buildup_rate), consumed)
}

fn accumulator_parser<T>(gene: &[AminoAcid], entity: Entity, mut commands: Commands) -> usize
where
    T: Send,
    T: Sync,
    T: Debug,
    T: 'static,
{
    let (accumulator, consumed) = accumulator_sequence_parser::<T>(gene);
    println!("Adding in an accumulator");
    //commands.get_entity(entity).unwrap().insert(accumulator);

    commands.spawn(accumulator);
    consumed
}

#[derive(Component, Default, Debug, HashedTypeDef)]
pub struct Accumulator<T>
where
    T: Send,
    T: Sync,
    T: Debug,
{
    pub level: u32,
    pub buildup_rate: u32,
    pub _phantom: PhantomData<T>,
}

impl<T> Accumulator<T>
where
    T: Send,
    T: Sync,
    T: Debug,
{
    pub fn new(level: u32, buildup_rate: u32) -> Self {
        Self {
            level,
            buildup_rate,
            _phantom: PhantomData::default(),
        }
    }
}

#[derive(HashedTypeDef)]
#[allow(dead_code)]
struct DopamineAccumulator(Accumulator<Dopamine>);

register_gene!(
    Accumulator<Dopamine>,
    { DopamineAccumulator::TYPE_HASH_NATIVE },
    accumulator_parser<Dopamine>,
    { crate::config::PROMOTER_SIZE }
);

#[derive(HashedTypeDef)]
#[allow(dead_code)]
struct SeratoninAccumulator(Accumulator<Seratonin>);

register_gene!(
    Accumulator<Seratonin>,
    { SeratoninAccumulator::TYPE_HASH_NATIVE },
    accumulator_parser<Seratonin>,
    { crate::config::PROMOTER_SIZE }
);

#[derive(HashedTypeDef)]
#[allow(dead_code)]
struct NorepinephrineAccumulator(Accumulator<Norepinephrine>);

register_gene!(
    Accumulator<Norepinephrine>,
    { NorepinephrineAccumulator::TYPE_HASH_NATIVE },
    accumulator_parser<Norepinephrine>,
    { crate::config::PROMOTER_SIZE }
);

#[cfg(test)]
mod test {
    use gene_traits::amino_acid::AminoAcid;

    use crate::components::Dopamine;

    use super::accumulator_sequence_parser;

    #[test]
    pub fn parse_accumulator_gene() {
        let expected = 4;

        let sequence = [
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
        ];

        let (accumulator, consumed) = accumulator_sequence_parser::<Dopamine>(&sequence);

        assert_eq!(accumulator.buildup_rate, expected);
        assert_eq!(consumed, 8);
    }

    #[test]
    pub fn parse_accumulator_gene_no_tata() {
        let expected = 4;

        let sequence = [AminoAcid::R, AminoAcid::R, AminoAcid::R, AminoAcid::R];

        let (accumulator, _) = accumulator_sequence_parser::<Dopamine>(&sequence);

        assert_eq!(accumulator.buildup_rate, expected);
    }

    #[test]
    pub fn parse_accumulator_gene_partial_tata() {
        let expected = 4;

        let sequence = [
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
        ];

        let (accumulator, consumed) = accumulator_sequence_parser::<Dopamine>(&sequence);

        assert_eq!(accumulator.buildup_rate, expected);
        assert_eq!(consumed, 7);
    }

    #[test]
    pub fn parse_accumulator_gene_invalid_tata() {
        let expected = 6;

        let sequence = [
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::R,
            AminoAcid::R,
        ];

        let (accumulator, consumed) = accumulator_sequence_parser::<Dopamine>(&sequence);

        assert_eq!(accumulator.buildup_rate, expected);
        assert_eq!(consumed, 9);
    }

    #[test]
    pub fn parse_accumulator_gene_multi() {
        let expected = 4;

        let sequence = [
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::R,
            AminoAcid::R,
        ];

        let (accumulator, consumed) = accumulator_sequence_parser::<Dopamine>(&sequence);

        assert_eq!(accumulator.buildup_rate, expected);
        assert_eq!(consumed, 8);
    }
}
