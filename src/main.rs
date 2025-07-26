use bevy::prelude::*;
use components::accumulator::Accumulator;
use evalexpr::build_operator_tree;
use gene_traits::{
    amino_acid::AminoAcid,
    dna::{get_hash, get_header},
};
use hashed_type_def::HashedTypeDef;

use std::fmt::Debug;

mod component_register;
mod components;
mod systems;

use crate::{
    component_register::ComponentRegister,
    config::PROMOTER_SIZE,
    systems::neuron_updates::{accumulator_buildup, receptor, update_neuron, update_synapse},
};
use crate::{components::*, systems::neurotransmitter_updates::update_dopamine};

mod config;

fn startup(mut commands: Commands) {
    for c in inventory::iter::<ComponentRegister<PROMOTER_SIZE>> {
        println!("{:?}", c);
    }
    let activators = [0]
        .map(|_| {
            commands
                .spawn((
                    Neuron {
                        dopamine: 100,
                        ..default()
                    },
                    Activation {
                        activation: build_operator_tree("dopamine >= 100"),
                    },
                    Synapse { active: false },
                    Accumulator::<Dopamine>::new(100, 5),
                ))
                .id()
        })
        .to_vec();
    let dopamine_receptor = commands
        .spawn((Receptor::<Dopamine> {
            level: 0,
            connected_accumulators: activators,
            ..default()
        },))
        .id();

    commands.spawn((
        Neuron {
            dopamine_receptors: vec![dopamine_receptor],
            ..default()
        },
        Synapse { active: false },
        Activation {
            activation: build_operator_tree("dopamine >= 100"),
        },
        UpdateFunction::default(),
    ));
}

// Ok, activation triggers a release of neurotransmitters, and the receptors get them.  Ok, great.
// Now, the receiving neuron needs to decide what to do with that received signal.

// I guess there needs to be a neuron "body" component as well then?
// ... This is complicated.  There are several neurotransmitter interactions that can occur.  Each
// pair of neurotransmitters must have systems to define the interactions.  May have to limit the
// amount of interactions.
//
// fn update_internal_state<T>()

//#[derive(Component, HashedTypeDef)]
//struct DopamineAccumulator(Accumulator<Dopamine>);
//
//impl From<Accumulator<Dopamine>> for DopamineAccumulator {
//    fn from(value: Accumulator<Dopamine>) -> Self {
//        Self(value)
//    }
//}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                receptor::<Dopamine>,
                accumulator_buildup::<Dopamine>,
                update_synapse,
                update_dopamine,
                update_neuron,
            ),
        )
        .run();
}
