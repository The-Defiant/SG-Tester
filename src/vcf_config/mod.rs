use log::{error, info};
use serde::Deserialize;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use self::vcf_header_field::VcfHeaderField;

pub mod vcf_filter;
pub mod vcf_format;
pub mod vcf_header_field;
pub mod vcf_info;

#[derive(Debug, Deserialize)]
pub struct VcfConfig {
    #[serde(rename(deserialize = "INFO"))]
    pub info: Vec<vcf_info::VcfInfoField>,
    #[serde(rename(deserialize = "FORMAT"))]
    pub format: Vec<vcf_format::VcfFormatField>,
    #[serde(rename(deserialize = "FILTER"))]
    pub filter: Vec<vcf_filter::VcfFilterField>,
}

pub trait VcfHeader {
    fn get_info_rows(&self) -> Vec<u8>;
    fn get_format_rows(&self) -> Vec<u8>;
    fn get_filter_rows(&self) -> Vec<u8>;
}

impl VcfHeader for VcfConfig {
    fn get_info_rows(&self) -> Vec<u8> {
        let rows: &mut Vec<u8> = &mut Vec::new();
        for record in self.info.iter() {
            let row: &mut Vec<u8> = &mut record.get_header_row();
            rows.append(row);
        }
        rows.to_owned()
    }
    fn get_format_rows(&self) -> Vec<u8> {
        let rows: &mut Vec<u8> = &mut Vec::new();
        for record in self.format.iter() {
            let row: &mut Vec<u8> = &mut record.get_header_row();
            rows.append(row);
        }
        rows.to_owned()
    }
    fn get_filter_rows(&self) -> Vec<u8> {
        let rows: &mut Vec<u8> = &mut Vec::new();
        for record in self.filter.iter() {
            let row: &mut Vec<u8> = &mut record.get_header_row();
            rows.append(row);
        }
        rows.to_owned()
    }
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
