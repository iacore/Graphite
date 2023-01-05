use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, ts_rs::TS)]
#[impl_message(Message, BroadcastMessage, TriggerEvent)]
pub enum BroadcastEvent {
	DocumentIsDirty,
	ToolAbort,
	SelectionChanged,
}
