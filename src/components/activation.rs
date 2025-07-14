use bevy::ecs::component::Component;
use evalexpr::*;

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
