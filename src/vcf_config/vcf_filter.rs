use crate::vcf_config::vcf_header_field::VcfHeaderField;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct VcfFilterField {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Values"))]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

impl VcfHeaderField for VcfFilterField {
    fn id(&self) -> String {
        self.id.to_owned()
    }
    fn description(&self) -> String {
        self.description.to_owned()
    }
    fn other(&self) -> HashMap<String, String> {
        self.other.to_owned()
    }
    fn get_default_fields(&self) -> Vec<u8> {
        let default_fields = format!(
            "##FILTER=<ID=\"{}\",Description=\"{}\"",
            self.id(),
            self.description()
        );
        default_fields.as_bytes().to_vec()
    }
}
