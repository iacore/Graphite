use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[impl_message(Message, Debug)]
#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize, TS)]
pub enum DebugMessage {
	ToggleTraceLogs,
	MessageOff,
	MessageNames,
	MessageContents,
}
