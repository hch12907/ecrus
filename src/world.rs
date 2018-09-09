use std::collections::{ BTreeMap, HashMap };

use EcsError;
use { Component, Entity, HotComponent, System };
use { ComponentId, EntityId };

pub struct World {
    entities: Vec<Option<Entity>>,
    components: HashMap<EntityId, HashMap<ComponentId, Box<Component>>>,
    hot_components: Vec<BTreeMap<ComponentId, Box<Component>>>, // hot_components[entity][component]
    systems: Vec<System>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            hot_components: Vec::new(),
            systems: Vec::new(),
        }
    }

    pub fn register_entity(&mut self) -> EntityId {
        match self.entities.binary_search(&None) {
            Ok(x) => {
                self.entities[x] = Some(Entity::new(x+1));
                EntityId(x+1)
            },
            Err(_) => {
                let id = self.entities.len() + 1;
                self.entities.push(Some(Entity::new(id)));
                EntityId(id)
            },
        }
    }

    pub fn register_component<T>(&mut self, entity_id: EntityId, component: T) 
        -> Result<ComponentId, EcsError> where T: Component
    {
        let component_id = ComponentId::of::<T>();

        match self.entities.get_mut(entity_id.0) {
            Some(Some(ref mut entity)) if entity.contains_component::<T>() =>
                Err(EcsError::ComponentAlreadyExists(entity_id, component_id)),

            Some(Some(ref mut entity)) => {
                entity.components_mut().insert(component_id);

                self.components
                    .entry(entity_id)
                    .or_insert({
                        let mut map: HashMap<ComponentId, Box<Component>> = HashMap::new();
                        map.insert(component_id, Box::new(component));
                        map
                    });
                
                Ok(component_id)
            },

            _ => Err(EcsError::EntityNotFound(entity_id))
        }
    }

    pub fn register_hot_component<T>(&mut self, entity_id: EntityId, component: T)
        -> Result<ComponentId, EcsError> where T: HotComponent 
    {
        let component_id = ComponentId::of::<T>();

        match self.entities.get_mut(entity_id.0) {
            // Return an error if component exists in target entity
            Some(Some(ref mut entity)) if entity.contains_hot_component::<T>() =>
                Err(EcsError::ComponentAlreadyExists(entity_id, component_id)),

            // Otherwise insert the component
            Some(Some(ref mut entity)) => {
                // Add the component to the entity set (used for quick checks)
                entity.hot_components_mut().insert(component_id);
                
                // Add the component to the world set (data stored here)
                self.hot_components.reserve(entity_id.0);
                self.hot_components[entity_id.0] = BTreeMap::new();
                self.hot_components[entity_id.0].insert(component_id, Box::new(component));

                Ok(component_id)
            }

            _ => Err(EcsError::EntityNotFound(entity_id))
        }
    }
}