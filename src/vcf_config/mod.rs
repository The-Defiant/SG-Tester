use log::{error, info};
use serde::Deserialize;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub mod vcf_filter;
pub mod vcf_format;
pub mod vcf_info;

#[derive(Debug, Deserialize)]
pub struct VcfConfig {
    #[serde(rename(deserialize = "INFO"))]
    pub info: Vec<vcf_info::VcfInfoConfig>,
    #[serde(rename(deserialize = "FORMAT"))]
    pub format: Vec<vcf_format::VcfFormatConfig>,
    #[serde(rename(deserialize = "FILTER"))]
    pub filter: Vec<vcf_filter::VcfFilterConfig>,
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
