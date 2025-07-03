use bevy::prelude::*;
use components::accumulator::Accumulator;
use evalexpr::build_operator_tree;
use gene_traits::Nucleotide;

use std::fmt::Debug;

mod components;
use crate::components::*;

mod config;

struct GeneRegister<const N: usize> {
    promoter: [Nucleotide; N],
    type_str: &'static str,
    parser: fn(&[Nucleotide], Commands),
}

inventory::collect!(GeneRegister<4>);

fn startup(mut commands: Commands) {
    let activators = [0]
        .map(|_| {
            commands
                .spawn((
                    Neuron {
                        dopamine: 100,
                        ..default()
                    },
                    Activation {
                        activation: build_operator_tree( "dopamine >= 100"),
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

fn update_synapse(mut query: Query<(&mut Neuron, &mut Synapse, &Activation)>) {
    for (mut neuron, mut synapse, activation) in query.iter_mut() {
        println!("Updating synapse");
        synapse.active = activation.get_activation(&mut neuron);
        if synapse.active {
            neuron.dopamine = 0;
        }
        println!("Synapse is active? {}", synapse.active);
    }
}

fn receptor<T>(
    mut accumulators: Query<(&Synapse, &mut Accumulator<T>)>,
    mut receptors: Query<&mut Receptor<T>>,
) where
    T: Send + 'static,
    T: Sync,
    T: Debug,
{
    // Well... this works, kinda, but it's not physically correct.  Need a way to "attract"
    // neurotransmitters to receptors.  This just assumes all neurotransmitters get taken up by all
    // receptors.  This is not reality, but kind of works for now.  Only for now, replace this.
    // TODO: Make this more physically accurate.
    for mut receptor in receptors.iter_mut() {
        let activation_sum: u32 = receptor
            .connected_accumulators
            .iter()
            .map(|&e| accumulators.get(e).unwrap())
            .filter(|(s, _)| s.active)
            .map(|(_, a)| a.level)
            .sum();
        receptor.level = activation_sum;
        println!("receptor level = {}", receptor.level);
    }

    // Consume the neurotransmitter
    for (_, mut activator) in accumulators.iter_mut() {
        activator.level = 0;
    }
}

fn accumulator_buildup<T>(mut accumulators: Query<&mut Accumulator<T>>)
where
    T: Send + 'static,
    T: Sync,
    T: Debug,
{
    for mut accumulator in accumulators.iter_mut() {
        accumulator.level += accumulator.buildup_rate;
    }
}

// TODO: This should really be made into a macro, all neurotransmitters will do exactly the same things
// just updating different fields inside the neuron
fn update_dopamine(mut neurons: Query<&mut Neuron>, receptors: Query<&Receptor<Dopamine>>) {
    for mut neuron in neurons.iter_mut() {
        neuron.dopamine += neuron
            .dopamine_receptors
            .iter()
            .map(|&e| receptors.get(e).unwrap())
            .map(|d| d.level)
            .sum::<u32>();
        println!("Dopamine level in neuron: {:?}", neuron.dopamine);
        // There also has to be some internal buildup of the various neurotransmitters
    }
}

fn update_neuron(mut neurons: Query<(&UpdateFunction, &mut Neuron)>) {
    for (update, mut neuron) in neurons.iter_mut() {
        (update.func)(&mut neuron); // This has to be some arbitrary function that gets modified by genetics
    }
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
