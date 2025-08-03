use bevy::ecs::{
    component::Component,
    system::{Commands, EntityCommands},
};
use evalexpr::*;
use gene_traits::amino_acid::AminoAcid;

use crate::config::PROMOTER_SIZE;

use super::Neuron;

#[derive(Component)]
pub struct Activation {
    // Amazingly, there's an expression evaluator already.  Just need to parse the genes to strings...
    pub activation: evalexpr::EvalexprResult<Node<DefaultNumericTypes>, DefaultNumericTypes>,
}

impl Activation {
    pub fn get_activation(&self, neuron: &Neuron) -> bool {
        let context = context_map! {
            "dopamine" => int neuron.dopamine,
            "seratonin" => int neuron.seratonin,
            "norepinephrine" => int neuron.norepinephrine
        }
        .unwrap();
        self.activation
            .as_ref()
            .expect("There must be an activation function")
            .eval_boolean_with_context(&context)
            .expect("Function must return a valid boolean")
    }
}

fn activation_sequence_parser(sequence: &[AminoAcid], mut commands: EntityCommands) -> usize {
    let last_idx = {
        if let Some(last_idx) = sequence
            .windows(PROMOTER_SIZE)
            .position(|w| w.eq(&[AminoAcid::UNKNOWN; 4]))
        {
            last_idx
        } else {
            sequence.len() - 1
        }
    };

    let expr = sequence[0..last_idx]
        .windows(2)
        // So... we kind of have to protect against point mutations that should be harmless
        // somehow...
        .map(|acid: &[AminoAcid]| match acid {
            &[AminoAcid::A, AminoAcid::A] => "(",
            &[AminoAcid::A, AminoAcid::P] => ")",
            &[AminoAcid::A, AminoAcid::F] => "*",
            &[AminoAcid::A, AminoAcid::M] => "/",
            &[AminoAcid::A, AminoAcid::K] => "^",
            &[AminoAcid::A, AminoAcid::S] => "+",
            &[AminoAcid::A, AminoAcid::W] => "-",
            &[AminoAcid::A, AminoAcid::T] => "%",
            &[AminoAcid::A, AminoAcid::Y] => "<",
            &[AminoAcid::A, AminoAcid::V] => ">",
            &[AminoAcid::A, AminoAcid::L] => "==",
            &[AminoAcid::A, AminoAcid::H] => ">=",
            &[AminoAcid::A, AminoAcid::D] => "<=",
            &[AminoAcid::A, AminoAcid::N] => "!=",
            &[AminoAcid::A, AminoAcid::R] => "!",
            &[AminoAcid::A, AminoAcid::I] => "math::sin",
            &[AminoAcid::A, AminoAcid::C] => "math::cos",
            &[AminoAcid::A, AminoAcid::E] => "math::ln",
            &[AminoAcid::A, AminoAcid::Q] => "math::log",
            &[AminoAcid::A, AminoAcid::G] => "math::log2",
            &[AminoAcid::P, AminoAcid::A] => "0",
            &[AminoAcid::P, AminoAcid::P] => "1",
            &[AminoAcid::P, AminoAcid::F] => "2",
            &[AminoAcid::P, AminoAcid::M] => "3",
            &[AminoAcid::P, AminoAcid::K] => "4",
            &[AminoAcid::P, AminoAcid::S] => "5",
            &[AminoAcid::P, AminoAcid::W] => "6",
            &[AminoAcid::P, AminoAcid::T] => "7",
            &[AminoAcid::P, AminoAcid::Y] => "8",
            &[AminoAcid::P, AminoAcid::V] => "9",
            _ => "",
        });
    0
}
