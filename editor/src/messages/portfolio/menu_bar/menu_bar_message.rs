use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[remain::sorted]
#[impl_message(Message, PortfolioMessage, MenuBar)]
#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize, ts_rs::TS)]
pub enum MenuBarMessage {
	// Messages
	SendLayout,
}
