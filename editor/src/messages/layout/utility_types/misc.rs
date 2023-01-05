use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[remain::sorted]
#[derive(PartialEq, Clone, Debug, Hash, Eq, Copy, Serialize, Deserialize, TS)]
#[repr(u8)]
pub enum LayoutTarget {
	DialogDetails,
	DocumentBar,
	DocumentMode,
	LayerTreeOptions,
	MenuBar,
	NodeGraphBar,
	PropertiesOptions,
	PropertiesSections,
	ToolOptions,
	ToolShelf,
	WorkingColors,

	// KEEP THIS ENUM LAST
	// This is a marker that is used to define an array that is used to hold widgets
	#[remain::unsorted]
	LayoutTargetLength,
}
