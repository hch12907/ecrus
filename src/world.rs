use std::any::{ Any, TypeId };
use std::collections::BTreeMap;

use crate::component::*;
use crate::entity::*;
use crate::system::*;

pub struct World {
    current_id: usize,
    components: BTreeMap<ComponentId, Vec<(Entity, Box<dyn Component>)>>,
    systems: BTreeMap<SystemId, &'static [ComponentId]>,
    cached_intersection: BTreeMap<SystemId, Vec<(ComponentId, Vec<*mut dyn Component>)>>,
}

impl World {
    pub fn new() -> Self {
        World {
            current_id: 0,
            components: BTreeMap::new(),
            systems: BTreeMap::new(),
            cached_intersection: BTreeMap::new(),
        }
    }

    pub fn register_entity(&mut self) -> Entity {
        let entity = Entity { id: self.current_id };
        self.current_id += 1;
        entity
    }

    pub fn add_component<C>(&mut self, ent: &Entity, comp: C) 
        where C: Component 
    {
        // Invalidate the cache if the component structure is modified
        for (sys, comps) in self.systems.iter() {
            if comps.contains(&component_id::<C>()) {
                self.cached_intersection.remove(sys);
            }
        }

        self.components.entry(component_id::<C>())
            .or_insert_with(|| Vec::new())
            .push((ent.clone(), Box::new(comp)));
    }

    pub fn get_component<C>(&self, ent: &Entity) -> Option<&C>
        where C: Component 
    {
        let comp_set = self.components.get(&TypeId::of::<C>())?;
        for (e, c) in comp_set {
            if e == ent {
                return Any::downcast_ref::<C>(c.as_ref().as_any())
            }
        }
        None
    }

    pub fn get_component_mut<C>(&mut self, ent: &Entity) -> Option<&mut C>
        where C: Component 
    {
        let comp_set = self.components.get_mut(&TypeId::of::<C>())?;
        
        for (e, c) in comp_set {
            if e == ent {
                return Any::downcast_mut::<C>(c.as_mut().as_any_mut())
            }
        }

        None
    }

    fn find_intersection<S>(&mut self) -> Vec<(ComponentId, Vec<*mut dyn Component>)>
        where S: System
    {
        let needed = S::needed_components();

        // Find duplicates using the bruteforce method during debug mode
        // In practice systems don't require a gazillion of components, and
        // since this is done in debug only, I think we can safely ignore
        // the performance implications here.
        #[cfg(debug_assertions)]
        for (i, n) in needed.iter().enumerate() {
            for (i2, n2) in needed.iter().enumerate() {
                if n == n2 && i != i2 { 
                    panic!("duplicated needed components in {}", 
                        std::any::type_name::<S>()
                    );
                }
            }
        }

        if needed.len() == 0 { return Vec::new() }

        // Get initial matches
        let mut initial_matches = Vec::new();
        for n in needed {
            if let Some(c) = self.components.get_mut(&n) {
                if c.len() != 0 {
                    // UNSAFE: As we had made sure earlier, the components we
                    // get are unique and can be mutated independently
                    let c = c as *mut Vec<(Entity, Box<dyn Component>)>;
                    let c = unsafe { &mut *c };
                    initial_matches.push((n.clone(), c));
                }
            }
        };

        // Skip if we can't find matching entities
        if needed.len() != initial_matches.len() { return Vec::new() }
        initial_matches.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        // Find the intersection of all entites w/ needed components
        // Stores (TypeId, Vec<&mut Component>), intersection[i][n] refers
        // to the i-th component of n-th entity
        let mut intersection = Vec::new();

        for im in &initial_matches {
            intersection.push((im.0, Vec::new()));
        }

        let mut initial_matches_s = initial_matches.split_off(1);

        'outer: for (idx, (ent, comp)) in initial_matches[0].1.iter_mut().enumerate() {
            for im in &initial_matches_s {
                if (im.1)[idx].0 != *ent {
                    continue 'outer;
                }
            }
            
            intersection[0].1.push(comp.as_mut() as *mut _);
            
            for (i, im) in initial_matches_s.iter_mut().enumerate() {
                // UNSAFE: the inside of initial_matches_s is not modified,
                // despite the iter_mut().
                let ent = (im.1)[idx].1.as_mut() as *mut _;
                intersection[i + 1].1.push(ent)
            }
        };

        intersection
    }

    pub fn run_with<S>(&mut self)
        where S: System 
    {
        self.systems.entry(S::id())
            .or_insert(S::needed_components());

        let ci = if let Some(ci) = self.cached_intersection.get(&S::id()) {
            ci
        } else {
            let intersection = self.find_intersection::<S>();
            // No intersections, skip
            if intersection.is_empty() { return }
            self.cached_intersection.entry(S::id()).or_insert(intersection)
        };

        let comp_set = ComponentSet::new(ci);
        S::run(comp_set)
    }
}
