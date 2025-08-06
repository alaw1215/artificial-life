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

fn parse_amino_acids(sequence: &[AminoAcid]) -> String {
    let expr = sequence
        .windows(2)
        .enumerate()
        // Need to "chunk" read not read with a sliding window
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|tpl| tpl.1)
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
            &[AminoAcid::F, AminoAcid::A] => "dopamine",
            &[AminoAcid::F, AminoAcid::P] => "seratonin",
            &[AminoAcid::F, AminoAcid::F] => "norepinephrine",
            _ => "",
        })
        .collect();

    expr
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

    let formula = parse_amino_acids(&sequence[0..last_idx]);

    last_idx
}

#[cfg(test)]
mod tests {
    use evalexpr::{
        DefaultNumericTypes, EvalexprError, HashMapContext, Value, build_operator_tree,
        context_map, eval_int_with_context,
    };
    use gene_traits::amino_acid::AminoAcid;

    use crate::components::activation::parse_amino_acids;

    #[test]
    fn parse_valid_function() {
        let mut context: evalexpr::HashMapContext<DefaultNumericTypes> = context_map! {
            "dopamine" => int 1,
            "seratonin" => int 1,
            "norepinephrine" => int 1
        }
        .unwrap();
        // want output formula 2*dopamine - 3*seratonin + norepinephrine
        // to get that formula, the amino acid chain should be PFAFFAAWPMAFFPASFF
        let amino_acids = [
            AminoAcid::P,
            AminoAcid::F,
            AminoAcid::A,
            AminoAcid::F,
            AminoAcid::F,
            AminoAcid::A,
            AminoAcid::A,
            AminoAcid::W,
            AminoAcid::P,
            AminoAcid::M,
            AminoAcid::A,
            AminoAcid::F,
            AminoAcid::F,
            AminoAcid::P,
            AminoAcid::A,
            AminoAcid::S,
            AminoAcid::F,
            AminoAcid::F,
        ];

        let expected = "2*dopamine-3*seratonin+norepinephrine";
        let actual = parse_amino_acids(&amino_acids);

        assert_eq!(expected, actual);

        let precompiled = build_operator_tree::<DefaultNumericTypes>(&actual).unwrap();

        assert_eq!(precompiled.eval_int_with_context(&context), Ok(0));
    }

    // I'm violating my own "do not test third party code" here, because this is actually very
    // important to know
    #[test]
    fn precompile_bad_expr() {
        let bad_expr = "(10*2+5";

        let mut corrected_expr = "".to_string();

        match build_operator_tree::<DefaultNumericTypes>(bad_expr) {
            Err(EvalexprError::UnmatchedLBrace) => {
                assert!(true);
                corrected_expr = bad_expr.to_owned() + ")";
            }
            _ => assert!(false),
        }

        if let Ok(_) = build_operator_tree::<DefaultNumericTypes>(&corrected_expr) {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
