use crate::helpers::extract_common_len;
use crate::vcf_config::vcf_header_field::VcfHeaderField;
use serde::Deserialize;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VcfFormatType {
    Integer(Vec<i32>),
    Float(Vec<f32>),
    Character(Vec<char>),
    String(Vec<String>),
    IntegerVec(Vec<Vec<i32>>),
    FloatVec(Vec<Vec<f32>>),
    CharacterVec(Vec<Vec<char>>),
    StringVec(Vec<Vec<String>>),
}

#[derive(Debug, Deserialize)]
pub struct VcfFormatField {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Values"))]
    pub values: VcfFormatType,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

impl VcfHeaderField for VcfFormatField {
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
            "##FORMAT=<ID=\"{}\",Description=\"{}\",Number=\"{}\",Type=\"{}\"",
            &self.id(),
            &self.description(),
            &self.number().to_string(),
            &self.value_type()
        );
        default_fields.as_bytes().to_vec()
    }
    fn number(&self) -> u8 {
        let values = &self.values;
        match values {
            VcfFormatType::Integer(_) => 1,
            VcfFormatType::Float(_) => 1,
            VcfFormatType::String(_) => 1,
            VcfFormatType::Character(_) => 1,
            VcfFormatType::IntegerVec(values) => extract_common_len(values),
            VcfFormatType::FloatVec(values) => extract_common_len(values),
            VcfFormatType::StringVec(values) => extract_common_len(values),
            VcfFormatType::CharacterVec(values) => extract_common_len(values),
        }
    }
    fn value_type(&self) -> String {
        let values = &self.values;
        let value_type = match values {
            VcfFormatType::Integer(_) => "Integer",
            VcfFormatType::Float(_) => "Float",
            VcfFormatType::String(_) => "String",
            VcfFormatType::Character(_) => "Character",
            VcfFormatType::IntegerVec(_) => "Integer",
            VcfFormatType::FloatVec(_) => "Float",
            VcfFormatType::StringVec(_) => "String",
            VcfFormatType::CharacterVec(_) => "String",
        };
        String::from(value_type)
    }
}
