use std::any::{ Any, TypeId };

pub type ComponentId = TypeId;

pub trait Component : Any + AsAny { 
    fn component_id(&self) -> ComponentId {
        TypeId::of::<Self>()
    }
}

pub trait HotComponent : Component {
    fn component_id(&self) -> ComponentId {
        TypeId::of::<Self>()
    }
}

pub trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: Component> AsAny for T {
    // Rust, as of now, does not support type upcasting i.e. casting derived-
    // trait objects into base-trait objects. This is a workaround.
    fn as_any(&self) -> &Any {
        self
    }
}