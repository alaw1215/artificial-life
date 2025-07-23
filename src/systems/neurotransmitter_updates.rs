use bevy::ecs::system::Query;

use crate::components::{Dopamine, Neuron, Receptor};

// TODO: This should really be made into a macro, all neurotransmitters will do exactly the same things
// just updating different fields inside the neuron
pub fn update_dopamine(mut neurons: Query<&mut Neuron>, receptors: Query<&Receptor<Dopamine>>) {
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
