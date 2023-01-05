use crate::messages::portfolio::utility_types::Platform;
use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[impl_message(Message, Globals)]
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, TS)]
pub enum GlobalsMessage {
	SetPlatform { platform: Platform },
}
