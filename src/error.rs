use std::error::Error;
use std::fmt::Error as FormatError;
use std::fmt::{ Display, Formatter };

use { ComponentId, EntityId };

#[derive(Clone, Debug)]
pub enum EcsError {
    ComponentAlreadyExists(EntityId, ComponentId),
    EntityNotFound(EntityId),
}

impl Display for EcsError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            EcsError::ComponentAlreadyExists(eid, cid) => 
                write!(fmt, "component #{:?} already exists in entity #{}", cid, eid),
                
            EcsError::EntityNotFound(id) => 
                write!(fmt, "entity #{} does not exist", id),
        }
    }
}

impl Error for EcsError {}