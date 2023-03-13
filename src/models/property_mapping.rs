use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct PropertyMapping {
    pub key: String,
    pub mapping_type: String,
    pub value_location: String,
}

