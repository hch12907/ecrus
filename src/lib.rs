mod component;
mod entity;
mod error;
mod system;
mod world;

// Enums
pub use error::EcsError;

// Structs
pub use entity::Entity as Entity;
pub use system::System as System;
pub use world::World as World;

// Traits
pub use component::AsAny as AsAny;
pub use component::Component as Component;
pub use component::HotComponent as HotComponent;

// Typedefs
pub use component::ComponentId as ComponentId;
pub use entity::EntityId as EntityId;