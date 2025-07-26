use bevy::ecs::component::Component;
use gene_traits::amino_acid::AminoAcid;

#[derive(Component)]
pub struct AminoAcidReader {
    pub strand: Vec<AminoAcid>,
}
