use bevy::ecs::{entity::Entity, system::Commands};
use gene_traits::amino_acid::AminoAcid;
use inventory;

/**
 * ComponentRegister registers the promoter sequence, a descriptive type name,
 * and a parsing function that takes the AminoAcid sequence and outputs the number of Amino Acids consumed during parsing
 */
#[derive(Debug)]
pub struct ComponentRegister<const N: usize> {
    pub header: [AminoAcid; N],
    pub type_hash: u128,
    pub type_str: &'static str,
    /**
     * Parser must return the number of amino acids consumed so that a long multi-gene strand can be processed
     */
    pub parser: fn(&[AminoAcid], Entity, Commands) -> usize,
}

inventory::collect!(ComponentRegister<4>);
