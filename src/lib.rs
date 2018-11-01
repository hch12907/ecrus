mod component;
mod entity;
mod errors;
mod system;
mod world;

// Reexport traits
pub use self::component::Component as Component;
pub use self::component::HotComponent as HotComponent;

// Reexport structs
pub use self::errors::EcrusError as EcrusError;
pub use self::entity::Entity as Entity;
pub use self::system::System as System;
pub use self::world::World as World;

// Reexport typedefs
pub use self::component::ComponentId as ComponentId;
pub use self::entity::EntityId as EntityId;