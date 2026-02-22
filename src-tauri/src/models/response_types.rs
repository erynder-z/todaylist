use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub notes_folder: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedNote {
    pub filename: String,
    pub formatted_name: String,
}
