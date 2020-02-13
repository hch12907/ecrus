use std::any::TypeId;

use crate::component::*;

pub type SystemId = TypeId;

pub trait System where Self: 'static + Sized {
    fn id() -> SystemId { TypeId::of::<Self>() }

    fn needed_components() -> &'static [ComponentId];

    fn run(comps: ComponentSet);
}
