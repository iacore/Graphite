use derivative::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Derivative, Debug, Default, PartialEq, Eq, TS)]
pub struct IconLabel {
	pub icon: String,

	pub disabled: bool,

	pub tooltip: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Separator {
	pub direction: SeparatorDirection,

	#[serde(rename = "type")]
	pub separator_type: SeparatorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub enum SeparatorDirection {
	Horizontal,
	Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub enum SeparatorType {
	Related,
	Unrelated,
	Section,
	List,
}

#[derive(Clone, Serialize, Deserialize, Derivative, Debug, PartialEq, Eq, Default, TS)]
pub struct TextLabel {
	pub disabled: bool,

	pub bold: bool,

	pub italic: bool,

	#[serde(rename = "tableAlign")]
	pub table_align: bool,

	pub multiline: bool,

	#[serde(rename = "minWidth")]
	pub min_width: u32,

	pub tooltip: String,

	// Body
	pub value: String,
}

// TODO: Add UserInputLabel
