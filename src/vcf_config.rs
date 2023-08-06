use log::{error, info};
use serde::Deserialize;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct VcfConfig {
    #[serde(rename(deserialize = "INFO"))]
    pub info: Vec<VcfInfoConfig>,
}

#[derive(Debug, Deserialize)]
pub struct VcfInfoConfig {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Number"))]
    pub number: i32,
    #[serde(rename(deserialize = "Type"))]
    pub t: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Source"))]
    pub source: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "Values"))]
    pub values: Vec<String>,
}

impl VcfConfig {
    fn from_path_buf(path: &PathBuf) -> String {
        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();
        content
    }
}
impl From<&PathBuf> for VcfConfig {
    fn from(path: &PathBuf) -> Self {
        let content = VcfConfig::from_path_buf(path);
        let de_config = serde_yaml::from_str(&content);
        match de_config {
            Ok(de_config) => {
                info!("Found configuration {:?}", &de_config);
                de_config
            }
            Err(err) => {
                error!("Failed to parse configuration");
                panic!("{}", err);
            }
        }
    }
}
