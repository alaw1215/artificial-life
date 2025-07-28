use bevy::{ecs::component::Component, prelude::Deref};
use gene_traits::amino_acid::AminoAcid;

#[derive(Component)]
pub struct AminoAcidReader;

#[derive(Component, Deref)]
pub struct AminoAcidChain(pub Vec<AminoAcid>);
