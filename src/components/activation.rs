use crate::components::expr_gene::{expr_from_amino_acids, last_idx_before_promoter};
use crate::config::PROMOTER_SIZE;
use crate::ComponentRegister;
use bevy::ecs::{
    component::Component,
    system::EntityCommands,
};
use evalexpr::*;
use gene_traits::amino_acid::AminoAcid;
use gene_traits::dna::get_header;
use gene_traits::mul;
use gene_traits::register_gene;
use hashed_type_def::HashedTypeDef;

use super::Neuron;

// Okay... this is kind of annoying, but it turns out not all types can just derive HashedTypeDef,
// and it is possible to create a 'tag' type that can act as a 'marker' for the type in the registration macro.
#[derive(HashedTypeDef)]
pub struct ActivationTag {}

#[derive(Component)]
pub struct Activation {
    // Amazingly, there's an expression evaluator already.  Just need to parse the genes to strings...
    pub activation: Node<DefaultNumericTypes>,
}

impl Activation {
    pub fn get_activation(&self, neuron: &Neuron) -> bool {
        let context = context_map! {
            "dopamine" => int neuron.dopamine,
            "serotonin" => int neuron.serotonin,
            "norepinephrine" => int neuron.norepinephrine
        }
            .unwrap();
        self.activation
            //.as_ref()
            //.expect("There must be an activation function")
            .eval_boolean_with_context(&context)
            .expect("Function must return a valid boolean")
    }

    pub fn sequence_parser(sequence: &[AminoAcid], mut commands: EntityCommands) -> usize {
        let last_idx = last_idx_before_promoter(sequence, PROMOTER_SIZE);

        let formula = expr_from_amino_acids(&sequence[0..last_idx]);

        let precompiled = build_operator_tree::<DefaultNumericTypes>(&formula)
            .expect("Failed to precompile activation function");

        let activation = Activation {
            activation: precompiled,
        };

        commands.insert(activation);

        last_idx
    }
}

pub fn activation_parser(sequence: &[AminoAcid], commands: EntityCommands) -> usize {
    Activation::sequence_parser(sequence, commands)
}

register_gene!(
    Activation,
    { ActivationTag::TYPE_HASH_NATIVE },
    activation_parser,
    { PROMOTER_SIZE }
);

#[cfg(test)]
mod tests {
    use evalexpr::{build_operator_tree, DefaultNumericTypes};

    use crate::components::Activation;

    use bevy::app::{App, Update};
    use bevy::ecs::world::World;

    use crate::components::{Neuron, Synapse};
    use crate::systems::neuron_updates::update_synapse;

    #[test]
    fn integration_activation_triggers_synapse_and_resets_dopamine() {
        let mut app = App::new();
        app.add_systems(Update, update_synapse);

        // Activation formula: dopamine > 0
        let activation_true = Activation {
            activation: build_operator_tree::<DefaultNumericTypes>("dopamine>0").unwrap(),
        };

        // Entity where activation should be true
        app.world_mut().spawn((
            Neuron {
                dopamine: 5,
                ..Default::default()
            },
            Synapse::default(),
            activation_true,
        ));

        // Activation formula: dopamine > 0 (same), but dopamine is 0, so false
        let activation_false = Activation {
            activation: build_operator_tree::<DefaultNumericTypes>("dopamine>0").unwrap(),
        };

        // Entity where activation should be false
        app.world_mut().spawn((
            Neuron {
                dopamine: 0,
                ..Default::default()
            },
            Synapse::default(),
            activation_false,
        ));

        // Run the system
        app.update();

        // Verify results
        let world: &mut World = app.world_mut();
        let mut query = world.query::<(&Neuron, &Synapse)>();
        let mut saw_true = false;
        let mut saw_false = false;

        for (neuron, synapse) in query.iter(app.world()) {
            if synapse.active {
                saw_true = true;
                assert_eq!(
                    neuron.dopamine, 0,
                    "Active synapse should reset dopamine to 0"
                );
            } else {
                saw_false = true;
                assert_eq!(
                    neuron.dopamine, 0,
                    "Inactive synapse should leave dopamine unchanged (expected 0)"
                );
            }
        }

        assert!(saw_true, "No entity had an active synapse");
        assert!(saw_false, "No entity had an inactive synapse");
    }
}
