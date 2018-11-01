use std::any::Any;

use crate::{ Component, HotComponent };
use crate::{ ComponentId, EntityId };
use crate::{ EcrusError, Entity, System };

pub struct World {
    entities: Vec<Entity>,
    systems: Vec<System>,
    latest_id: EntityId,
}

impl World {
    pub fn add_entity(
        &mut self, 
        cold: Box<[Box<dyn Component>]>, 
        hot: Box<[Box<dyn HotComponent>]>) -> Result<EntityId, EcrusError>
    {
        let mut entity = Entity::new(self.latest_id);
        
        for comp in Vec::from(cold) { 
            entity.add_component(comp)?;
        }

        for comp in Vec::from(hot) {
            entity.add_hot_component(comp)?;
        }

        self.entities.push(entity);

        self.latest_id += 1;

        Ok(self.latest_id - 1)
    }
}
