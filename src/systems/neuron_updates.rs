use bevy::ecs::system::Query;
use std::fmt::Debug;

use crate::components::{
    accumulator::Accumulator, Activation, Neuron, NeuronUpdater, Receptor, Synapse, UpdateFunction
};

// The neuron should update in a way that neurotransmitters are both accepted by receptors, and degraded by some internal function
pub fn update_neuron(mut neurons: Query<(&UpdateFunction, &mut Neuron)>) {
    for (update, mut neuron) in neurons.iter_mut() {
        update.eval(&mut neuron); // This has to be some arbitrary function that gets modified by genetics
    }
}

pub fn update_synapse(mut query: Query<(&mut Neuron, &mut Synapse, &Activation)>) {
    for (mut neuron, mut synapse, activation) in query.iter_mut() {
        println!("Updating synapse");
        synapse.active = activation.get_activation(&mut neuron);
        if synapse.active {
            neuron.dopamine = 0;
        }
        println!("Synapse is active? {}", synapse.active);
    }
}

pub fn receptor<T>(
    mut accumulators: Query<(&Synapse, &mut Accumulator<T>)>,
    mut receptors: Query<(&mut Receptor<T>, &mut Neuron)>,
) where
    T: Send + 'static,
    T: Sync,
    T: Debug,
    T: NeuronUpdater
{
    // Well... this works, kinda, but it's not physically correct.  Need a way to "attract"
    // neurotransmitters to receptors.  This just assumes all neurotransmitters get taken up by all
    // receptors.  This is not reality, but kind of works for now.  Only for now, replace this.
    // TODO: Make this more physically accurate.
    for (mut receptor, mut neuron) in receptors.iter_mut() {
        let activation_sum: u32 = receptor
            .connected_accumulators
            .iter()
            .map(|&e| accumulators.get(e).unwrap())
            .filter(|(s, _)| s.active)
            .map(|(_, a)| a.level)
            .sum();
        receptor.level = activation_sum;
        T::update(receptor.level, &mut neuron);
        println!("receptor level = {}", receptor.level);
    }

    // Consume the neurotransmitter
    for (_, mut activator) in accumulators.iter_mut() {
        activator.level = 0;
    }
}

pub fn accumulator_buildup<T>(mut accumulators: Query<&mut Accumulator<T>>)
where
    T: Send + 'static,
    T: Sync,
    T: Debug,
{
    for mut accumulator in accumulators.iter_mut() {
        accumulator.level += accumulator.buildup_rate;
    }
}
