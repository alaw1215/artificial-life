use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component, Default, Debug)]
pub struct Neuron {
    pub dopamine: u32,
    pub seratonin: u32,
    pub norepinephrine: u32,
    pub dopamine_receptors: Vec<Entity>,
    pub seratonin_receptors: Vec<Entity>,
    pub norepinephrine_receptors: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct UpdateFunction {
    pub func: fn(&mut Neuron) -> bool,
}

impl Default for UpdateFunction {
    fn default() -> Self {
        Self { func: |_| false }
    }
}

// A synapse is in itself a separate entity, and each neuron can contain the same synapses for
// different purposes.  This allows synapse state to be propagated.
#[derive(Component, Default, Debug)]
pub struct Synapse {
    pub active: bool,
}
