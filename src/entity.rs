use std::any::{ Any, TypeId };
use std::collections::HashSet;
use std::fmt::{ Display, Result as FormatResult, Formatter };

use ComponentId;
use { Component, HotComponent };

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct EntityId(pub(crate) usize);

impl Display for EntityId {
    fn fmt(&self, fmt: &mut Formatter) -> FormatResult {
        write!(fmt, "{}", self.0)
    }
}

pub struct Entity {
    id: EntityId,
    components: HashSet<ComponentId>,
    hot_components: HashSet<ComponentId>,
}

impl Entity {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id: EntityId(id),
            components: HashSet::new(),
            hot_components: HashSet::new(),
        }
    }

    pub fn components(&self) -> Box<[&TypeId]> {
        let keys = self.components.iter();
        keys.collect::<Vec<_>>().into_boxed_slice()
    }

    pub(crate) fn components_mut(&mut self) -> &mut HashSet<TypeId> {
        &mut self.components
    }

    pub fn hot_components(&self) -> Box<[&TypeId]> {
        let keys = self.hot_components.iter();
        keys.collect::<Vec<_>>().into_boxed_slice()
    }

    pub(crate) fn hot_components_mut(&mut self) -> &mut HashSet<TypeId> {
        &mut self.hot_components
    }

    pub fn contains_component<T>(&self) -> bool 
        where T: Component
    {
        self.components.contains(&TypeId::of::<T>())
    }

    pub fn contains_hot_component<T>(&self) -> bool 
        where T: HotComponent
    {
        self.components.contains(&TypeId::of::<T>())
    }

    pub fn entity_id(&self) -> usize {
        self.id.0
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Entity) -> Option<::std::cmp::Ordering> {
        (self.id).partial_cmp(&other.id)
    }
}

impl Eq for Entity {}

impl Ord for Entity {
    fn cmp(&self, other: &Entity) -> ::std::cmp::Ordering {
        (self.id).cmp(&other.id)
    }
}