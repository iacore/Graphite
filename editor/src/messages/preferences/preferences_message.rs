use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[impl_message(Message, Preferences)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, TS)]
pub enum PreferencesMessage {
	Load { preferences: String },
	ResetToDefaults,

	ImaginateRefreshFrequency { seconds: f64 },
	ImaginateServerHostname { hostname: String },
}
