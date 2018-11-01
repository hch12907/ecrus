use std::any::{ Any, TypeId };

pub type ComponentId = TypeId;

/// A cold component. Use this for components that are not frequently accessed.
pub trait Component : Any + Send {
    /// Obtains the `ComponentId` of the component.
    /// Note: *It is highly discouraged for one to override this function.*
    fn component_id(&self) -> ComponentId {
        TypeId::of::<Self>()
    }
}

/// A hot component. Use this for components that are frequently accessed.
pub trait HotComponent : Any + Send { 
    /// Obtains the `ComponentId` of the component.
    /// Note: *It is highly discouraged for one to override this function.*
    fn component_id(&self) -> ComponentId {
        TypeId::of::<Self>()
    }
}
