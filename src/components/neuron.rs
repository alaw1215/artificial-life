use bevy::ecs::{component::Component, entity::Entity};
use evalexpr::{build_operator_tree, context_map, DefaultNumericTypes, Node};
use bevy::ecs::system::EntityCommands;
use gene_traits::amino_acid::AminoAcid;
use crate::config::PROMOTER_SIZE;
use crate::components::expr_gene::{expr_from_amino_acids, last_idx_before_promoter};

#[derive(Component, Default, Debug)]
pub struct Neuron {
    pub dopamine: u32,
    pub serotonin: u32,
    pub norepinephrine: u32,
    pub dopamine_receptors: Vec<Entity>,
    pub serotonin_receptors: Vec<Entity>,
    pub norepinephrine_receptors: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct UpdateFunction {
    pub func: evalexpr::EvalexprResult<Node<DefaultNumericTypes>, DefaultNumericTypes>,
}

impl UpdateFunction {
    pub fn eval(&self, neuron: &mut Neuron) {
        let context : evalexpr::HashMapContext<DefaultNumericTypes> = context_map! {
            "dopamine" => int neuron.dopamine,
            "serotonin" => int neuron.serotonin,
            "norepinephrine" => int neuron.norepinephrine
        }
        .unwrap();

        // What exactly should I do here?
        // First, update the internal neurotransmitter levels based on the receptors... that I didn't take in here...
        let _ = &context;
    }

    // Wrapper provided to mirror Activation's API and reuse common logic if needed elsewhere
    fn parse_amino_acids(sequence: &[AminoAcid]) -> String {
        expr_from_amino_acids(sequence)
    }

    // Gene parser for UpdateFunction using the shared helpers
    pub fn sequence_parser(sequence: &[AminoAcid], mut commands: EntityCommands) -> usize {
        let last_idx = last_idx_before_promoter(sequence, PROMOTER_SIZE);

        let formula = expr_from_amino_acids(&sequence[0..last_idx]);

        let precompiled = build_operator_tree::<DefaultNumericTypes>(&formula)
            .expect("Failed to precompile update function");

        let update_fn = UpdateFunction { func: Ok(precompiled) };

        commands.insert(update_fn);

        last_idx
    }
}

impl Default for UpdateFunction {
    fn default() -> Self {
        Self { func: build_operator_tree::<DefaultNumericTypes>("") }
    }
}

// A synapse is in itself a separate entity, and each neuron can contain the same synapses for
// different purposes.  This allows the synapse state to be propagated.
#[derive(Component, Default, Debug)]
pub struct Synapse {
    pub active: bool,
}
