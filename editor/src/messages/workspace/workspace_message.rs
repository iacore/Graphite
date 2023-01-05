use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[remain::sorted]
#[impl_message(Message, Workspace)]
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, TS)]
pub enum WorkspaceMessage {
	// Messages
	NodeGraphToggleVisibility,
}
