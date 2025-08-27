use bevy::ecs::{
    system::{Commands, Query},
};
use generic_levenshtein;
use crate::{
    component_register::ComponentRegister,
    components::amino_acid_reader::AminoAcidChain,
    config::PROMOTER_SIZE,
};

pub fn parse_amino_acid_strand(
    query: Query<&AminoAcidChain>,
    mut commands: Commands,
) {
    for acid in query.iter() {
        let mut strand_index = 0;
        let mut e = commands.spawn_empty();
        while strand_index < acid.len() {
            let slice = &acid[strand_index..];
            let mut strand_window = slice.windows(PROMOTER_SIZE).enumerate();
            let mut distance = PROMOTER_SIZE + 1;
            let mut parser = None;
            let mut window_idx = 0;
            // Search through the strand for a known component header
            while let Some((idx, current_window)) = strand_window.next() {
                for c in inventory::iter::<ComponentRegister<PROMOTER_SIZE>>::iter() {
                    let current_distance = generic_levenshtein::distance(current_window, &c.header);
                    if (current_distance < PROMOTER_SIZE / 2) && (current_distance < distance) {
                        distance = current_distance;
                        parser = Some(c.parser);
                        window_idx = idx;
                    }
                    // Found an exact match, bail
                    if distance == 0 {
                        break;
                    }
                }
            }
            if parser.is_some() {
                strand_index = window_idx * PROMOTER_SIZE + PROMOTER_SIZE;
                strand_index += parser.unwrap()(&acid[strand_index..], e.reborrow());
            } else {
                break;
            }
        }
    }
}

#[macro_export]
macro_rules! amino_acid_header {
    ($ty:ty) => {{
        const HASH: u128 = get_hash::<$ty>();
        get_header::<4, 12, HASH>()
    }}
}

#[cfg(test)]
mod test {
    use bevy::{
        app::{App, Update},
        ecs::world::World,
    };
    use gene_traits::amino_acid::AminoAcid;
    use gene_traits::dna::{get_hash, get_header};
    use crate::{
        components::{
            Norepinephrine,
            accumulator::Accumulator,
            amino_acid_reader::{AminoAcidChain, AminoAcidReader},
        },
        systems::ribosome::parse_amino_acid_strand,
    };
    use crate::components::Activation;
    use crate::components::activation::ActivationTag;

    #[test]
    fn parsed_valid_amino_acid_strand() {
        let sequence = [
            AminoAcid::D,
            AminoAcid::A,
            AminoAcid::P,
            AminoAcid::W,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
        ];

        let mut app = App::new();

        app.add_systems(Update, parse_amino_acid_strand);

        app.world_mut()
            .spawn((AminoAcidReader, AminoAcidChain(sequence.to_vec())));

        app.update();

        let world: &mut World = app.world_mut();

        let mut query = world.query::<&Accumulator<Norepinephrine>>();

        assert!(query.iter(app.world()).len() > 0);
    }

    #[test]
    fn parsed_valid_activation() {
        let header = amino_acid_header!(ActivationTag);
        let sequence = [
            header[0],
            header[1],
            header[2],
            header[3],
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
        ];

        let mut app = App::new();

        app.add_systems(Update, parse_amino_acid_strand);

        app.world_mut()
            .spawn((AminoAcidReader, AminoAcidChain(sequence.to_vec())));

        app.update();

        let world: &mut World = app.world_mut();

        let mut query = world.query::<&Activation>();

        assert!(query.iter(app.world()).len() > 0);
    }

    #[test]
    fn bad_header_amino_acid_strand() {
        // In order to fail to match, there must be 2 or more incorrect amino acids
        let sequence = [
            AminoAcid::A,
            AminoAcid::R,
            AminoAcid::P,
            AminoAcid::W,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::R,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
        ];

        let mut app = App::new();

        app.add_systems(Update, parse_amino_acid_strand);

        app.world_mut()
            .spawn((AminoAcidReader, AminoAcidChain(sequence.to_vec())));

        app.update();

        let world: &mut World = app.world_mut();

        let mut query = world.query::<&Accumulator<Norepinephrine>>();

        assert!(query.iter(app.world()).len() == 0);
    }
}
