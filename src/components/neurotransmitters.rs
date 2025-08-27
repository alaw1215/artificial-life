use hashed_type_def::HashedTypeDef;
use crate::components::Neuron;

pub trait NeuronUpdater {
    fn update(level: u32, neuron: &mut Neuron);
}

#[derive(Default, Debug, HashedTypeDef)]
pub struct Dopamine;

impl NeuronUpdater for Dopamine {
    fn update(level: u32, neuron: &mut Neuron) {
        neuron.dopamine += level;
    }
}

#[derive(Default, Debug, HashedTypeDef)]
pub struct Serotonin;

impl NeuronUpdater for Serotonin {
    fn update(level: u32, neuron: &mut Neuron) {
        neuron.serotonin += level;
    }
}

#[derive(Default, Debug, HashedTypeDef)]
pub struct Norepinephrine;

impl NeuronUpdater for Norepinephrine {
    fn update(level: u32, neuron: &mut Neuron) {
        neuron.dopamine += level;
    }
}
