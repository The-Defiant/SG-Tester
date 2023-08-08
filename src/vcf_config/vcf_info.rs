use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct VcfInfoConfig {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Values"))]
    pub values: VcfConfigInfoType,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VcfConfigInfoType {
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
