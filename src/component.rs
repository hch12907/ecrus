use std::any::{ Any, TypeId };

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
