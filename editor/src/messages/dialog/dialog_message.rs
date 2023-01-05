use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[remain::sorted]
#[impl_message(Message, Dialog)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, TS)]
pub enum DialogMessage {
	// Sub-messages
	#[remain::unsorted]
	#[child]
	ExportDialog(ExportDialogMessage),
	#[remain::unsorted]
	#[child]
	NewDocumentDialog(NewDocumentDialogMessage),
	#[remain::unsorted]
	#[child]
	PreferencesDialog(PreferencesDialogMessage),

	// Messages
	CloseAllDocumentsWithConfirmation,
	CloseDialogAndThen {
		followups: Vec<Message>,
	},
	DisplayDialogError {
		title: String,
		description: String,
	},
	RequestAboutGraphiteDialog,
	RequestAboutGraphiteDialogWithLocalizedCommitDate {
		localized_commit_date: String,
	},
	RequestComingSoonDialog {
		issue: Option<i32>,
	},
	RequestExportDialog,
	RequestNewDocumentDialog,
	RequestPreferencesDialog,
}
