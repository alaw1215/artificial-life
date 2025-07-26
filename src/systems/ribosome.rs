use bevy::ecs::system::{Commands, Query};
use generic_levenshtein;

use crate::{
    component_register::ComponentRegister, components::amino_acid_reader::AminoAcidReader,
    config::PROMOTER_SIZE,
};

pub fn parse_amino_acid_strand(query: Query<&AminoAcidReader>, mut commands: Commands) {
    for acid in query.iter() {
        let mut strand_index = 0;
        let e = commands.spawn_empty().id();
        println!("Spawned an entity we will add components to later");
        while strand_index < acid.strand.len() {
            let slice = &acid.strand[strand_index..];
            let mut strand_window = slice.windows(PROMOTER_SIZE).enumerate();
            let mut distance = PROMOTER_SIZE + 1;
            let mut parser = None;
            let mut window_idx = 0;
            // Search through the strand for a known component header
            while let Some((idx, current_window)) = strand_window.next() {
                println!("current window contains {:?}", current_window);
                for c in inventory::iter::<ComponentRegister<PROMOTER_SIZE>>::iter() {
                    println!("I'm looking for this header: {:?}", c.header);
                    let current_distance = generic_levenshtein::distance(current_window, &c.header);
                    println!("Current levenshtein distance is {}", current_distance);
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
                println!("I found a header in window {}", window_idx);
                strand_index = window_idx * PROMOTER_SIZE + PROMOTER_SIZE;
                println!("Passing strand slice {:?}", &acid.strand[strand_index..]);
                strand_index +=
                    parser.unwrap()(&acid.strand[strand_index..], e, commands.reborrow());
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        app::{App, Update},
        ecs::{system::Query, world::World},
    };
    use gene_traits::amino_acid::AminoAcid;

    use crate::{
        components::{Dopamine, accumulator::Accumulator, amino_acid_reader::AminoAcidReader},
        systems::ribosome::parse_amino_acid_strand,
    };

    fn test_query(query: Query<&Accumulator<Dopamine>>) {
        for a in query.iter() {
            println!("I have at least one accumulator");
        }
    }
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

        app.add_systems(Update, (parse_amino_acid_strand, test_query));

        let strand_id = app
            .world_mut()
            .spawn(AminoAcidReader {
                strand: sequence.to_vec(),
            })
            .id();

        //app.world_mut().spawn(Accumulator::<Dopamine>::new(0, 0));

        app.update();
        app.update();

        let world: &mut World = app.world_mut();

        let mut query = world.query::<&Accumulator<Dopamine>>();

        assert!(query.iter(app.world()).len() > 0);
    }
}
