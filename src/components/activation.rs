use bevy::ecs::component::Component;

use super::Neuron;

#[derive(Component)]
pub struct Activation {
    pub activation: fn(&Neuron) -> bool,
}

impl Activation {
    pub fn get_activation(&self, neuron: &Neuron) -> bool {
        (self.activation)(neuron)
    }
}
