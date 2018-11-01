use std::error::Error;
use std::fmt::{ Display, Result as FmtResult, Formatter };

use crate::{ ComponentId, EntityId };

#[derive(Debug)]
pub enum EcrusError {
    ComponentAlreadyRegistered {
        comp_id: ComponentId,
        entity_id: EntityId,
    },
}

impl Display for EcrusError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EcrusError::ComponentAlreadyRegistered { comp_id, entity_id } =>
                write!(f, "ecs: component {:?} already registered in {:?}", comp_id, entity_id)
        }
    }
}

impl Error for EcrusError {}