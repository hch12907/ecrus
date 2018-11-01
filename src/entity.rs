use std::any::{ Any, TypeId };
use std::collections::BTreeMap;

use crate::{ Component, ComponentId, EcrusError, HotComponent };

pub type EntityId = u32;

pub struct Entity {
    id: EntityId,
    cold_component: BTreeMap<ComponentId, Box<Any>>,
    hot_component: Vec<(ComponentId, Box<Any>)>,
}

impl Entity {
    pub(crate) fn new(id: EntityId) -> Self {
        Self {
            id,
            cold_component: BTreeMap::new(),
            hot_component: Vec::new(),
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.id
    }

    pub(crate) fn add_component(&mut self, data: Box<dyn Component>) -> Result<(), EcrusError>
    {
        let type_id = Component::component_id(data.as_ref());
        if !self.cold_component.contains_key(&type_id) {
            self.cold_component.insert(type_id, Box::new(data));
            Ok(())
        } else {
            Err(EcrusError::ComponentAlreadyRegistered { 
                comp_id: type_id,
                entity_id: self.id,
            })?
        }
    }

    pub(crate) fn add_hot_component(&mut self, data: Box<dyn HotComponent>) -> Result<(), EcrusError>
    {
        let type_id = HotComponent::component_id(data.as_ref());

        for (id, comp) in self.hot_component.iter_mut() {
            if *id == type_id {
                Err(EcrusError::ComponentAlreadyRegistered { 
                    comp_id: type_id,
                    entity_id: self.id,
                })?
            }
        }

        self.hot_component.push((type_id, Box::new(data)));
        
        Ok(())
    }

    pub fn get_component<T>(&self) -> Option<&T>
        where T: Component
    {
        self.cold_component
            .get(&TypeId::of::<T>())
            .map(|x| Any::downcast_ref::<T>(x.as_ref()).unwrap())
    }

    pub fn get_component_mut<T>(&mut self) -> Option<&mut T>
        where T: Component
    {
        self.cold_component
            .get_mut(&TypeId::of::<T>())
            .map(|x| Any::downcast_mut::<T>(x.as_mut()).unwrap())
    }

    pub fn get_hot_component<T>(&self) -> Option<&T>
        where T: HotComponent
    {
        let type_id = TypeId::of::<T>();

        self.hot_component
            .iter()
            .filter(|(id, _)| id != &type_id)
            .next()
            .map(|(_, com)| Any::downcast_ref(com.as_ref()).unwrap())
    }

    pub fn get_hot_component_mut<T>(&mut self) -> Option<&mut T>
        where T: HotComponent
    {
        let type_id = TypeId::of::<T>();

        self.hot_component
            .iter_mut()
            .filter(|(id, _)| id != &type_id)
            .next()
            .map(|(_, com)| Any::downcast_mut(com.as_mut()).unwrap())
    }
}
