use bevy::ecs::component::Component;

use super::Neuron;

#[derive(Component)]
pub struct Activation {
    // TODO: Keep track of the operators and factors instead of the function, as it is impossible
    // to capture the closure properly
    pub activation: fn(&Neuron) -> bool,
}

impl Activation {
    pub fn get_activation(&self, neuron: &Neuron) -> bool {
        (self.activation)(neuron)
    }
}
