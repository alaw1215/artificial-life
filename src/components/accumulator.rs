use bevy::ecs::system::Commands;
use bevy::prelude::Component;
use gene_traits::amino_acid::AminoAcid;
use gene_traits::{dna::Nucleotide, dna::get_promoter, register_gene};
use hashed_type_def::HashedTypeDef;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::GeneRegister;

use crate::neurotransmitters::*;

fn accumulator_sequence_parser<T>(gene: &[AminoAcid]) -> Accumulator<T>
where
    T: Send,
    T: Sync,
    T: Debug,
    T: 'static,
{
    let mut buildup_rate = 0;
    for i in 0..8 {
        let current_part: u32 = (Into::<u8>::into(gene[i]) as u32) << (i * 2);

        buildup_rate += current_part;
    }

    Accumulator::new(0, buildup_rate)
}

fn accumulator_parser<T>(gene: &[AminoAcid], mut commands: Commands)
where
    T: Send,
    T: Sync,
    T: Debug,
    T: 'static,
{
    let accumulator: Accumulator<T> = accumulator_sequence_parser(gene);
    commands.spawn(accumulator);
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

register_gene!(Accumulator<Dopamine>, accumulator_parser<Dopamine>, {
    crate::config::PROMOTER_SIZE
});

register_gene!(Accumulator<Seratonin>, accumulator_parser<Seratonin>, {
    crate::config::PROMOTER_SIZE
});

register_gene!(
    Accumulator<Norepinephrine>,
    accumulator_parser<Norepinephrine>,
    { crate::config::PROMOTER_SIZE }
);

#[cfg(test)]
mod test {
    use gene_traits::amino_acid::AminoAcid;

    use crate::components::Dopamine;

    use super::accumulator_sequence_parser;

    pub fn parse_accumulator_gene() {
        let expected = 4;

        let sequence = [AminoAcid::A, AminoAcid::R, AminoAcid::A, AminoAcid::A];

        let accumulator = accumulator_sequence_parser::<Dopamine>(&sequence);

        assert_eq!(accumulator.buildup_rate, expected);
    }
}
