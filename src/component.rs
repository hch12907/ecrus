use std::any::{ Any, TypeId };

use crate::bitset::BitSet;

pub type ComponentId = TypeId;

pub trait Component: Any + ComponentIntoAny + 'static {}

pub trait ComponentIntoAny {
    fn as_any(&self) -> &dyn Any;
    
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<C: Component> ComponentIntoAny for C {
    fn as_any(&self) -> &dyn Any { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub const fn component_id<C>() -> ComponentId 
    where C: Component
{
    TypeId::of::<C>()
}

pub struct ComponentSet<'a> {
    comps: &'a [(ComponentId, Vec<*mut dyn Component>)],
    obtained: BitSet,
}

impl<'a> ComponentSet<'a> {
    pub(crate) fn new(comps: &'a [(ComponentId, Vec<*mut dyn Component>)]) -> Self {
        Self {
            comps,
            obtained: BitSet::with_capacity_zeroed(comps.len()),
        }
    }

    pub fn get_components<C>(&mut self) -> Vec<&'a mut C>
        where C: Component
    {
        for (i, c) in self.comps.iter().enumerate() {
            if c.0 == component_id::<C>() {
                // panic if this component had already been obtained
                if self.obtained.get(i) {
                    panic!(
                        "double mut reference to component {}",
                        std::any::type_name::<C>()
                    )
                };

                self.obtained.set(i, true);
                
                let comp_ptr_vec = &c.1;
                let comp_mut_vec = comp_ptr_vec.iter()
                    .map(|comp_ptr| {
                        let deref = unsafe { &mut **comp_ptr };
                        let deref_any = deref.as_any_mut();
                        let downcasted = deref_any.downcast_mut::<C>();
                        downcasted.unwrap()
                    })
                    .collect::<Vec<_>>();

                return comp_mut_vec
            }
        }

        unreachable!();
    }
}
