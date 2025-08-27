use std::marker::PhantomData;

use bevy::ecs::{component::Component, entity::Entity};


#[derive(Component, Default)]
pub struct Receptor<T> {
    pub level: u32, // Is this necessary?  Should it be stored in another way?
    pub connected_accumulators: Vec<Entity>,
    pub _phantom: PhantomData<T>,
}
