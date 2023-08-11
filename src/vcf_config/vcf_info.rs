use crate::helpers::extract_common_len;
use crate::vcf_config::vcf_header_field::VcfHeaderField;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]

pub enum VcfInfoType {
    Integer(Vec<i32>),
    Float(Vec<f32>),
    Character(Vec<char>),
    String(Vec<String>),
    IntegerVec(Vec<Vec<i32>>),
    FloatVec(Vec<Vec<f32>>),
    CharacterVec(Vec<Vec<char>>),
    StringVec(Vec<Vec<String>>),
    Flag(),
}
#[derive(Debug, Deserialize)]
pub struct VcfInfoField {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Values"))]
    pub values: VcfInfoType,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

impl VcfHeaderField for VcfInfoField {
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
            "##INFO=<ID=\"{}\",Description=\"{}\",Number=\"{}\",Type=\"{}\"",
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
            VcfInfoType::Integer(_) => 1,
            VcfInfoType::Float(_) => 1,
            VcfInfoType::String(_) => 1,
            VcfInfoType::Character(_) => 1,
            VcfInfoType::IntegerVec(values) => extract_common_len(values),
            VcfInfoType::FloatVec(values) => extract_common_len(values),
            VcfInfoType::StringVec(values) => extract_common_len(values),
            VcfInfoType::CharacterVec(values) => extract_common_len(values),
            VcfInfoType::Flag() => 0,
        }
    }
    fn value_type(&self) -> String {
        let values = &self.values;
        let value_type = match values {
            VcfInfoType::Integer(_) => "Integer",
            VcfInfoType::Float(_) => "Float",
            VcfInfoType::String(_) => "String",
            VcfInfoType::Character(_) => "Character",
            VcfInfoType::IntegerVec(_) => "Integer",
            VcfInfoType::FloatVec(_) => "Float",
            VcfInfoType::StringVec(_) => "String",
            VcfInfoType::CharacterVec(_) => "String",
            VcfInfoType::Flag() => "Flag",
        };
        String::from(value_type)
    }
}
