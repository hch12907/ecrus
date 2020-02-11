use crate::component::*;

pub trait System {
    fn needed_components() -> &'static [ComponentId];

    fn run(comps: &mut [(ComponentId, Vec<&mut dyn Component>)]);
}
