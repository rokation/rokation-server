use serde::{Deserialize, Serialize};

use crate::{command::CommandResult, entity::EntitySnapshot, event::Event};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    CommandResult { result: CommandResult },
    Event { event: Event },
    Snapshot { entities: Vec<EntitySnapshot> },
}
