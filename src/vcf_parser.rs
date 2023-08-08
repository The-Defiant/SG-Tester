use log::info;

use crate::vcf_config::VcfConfig;

pub struct VcfParser {
    pub vcf_config: VcfConfig,
}

impl<'a> VcfParser {
    pub fn get_file_format() -> &'a [u8] {
        b"##fileformat=VCFv4.2\n"
    }

    pub fn get_info_fields(&self) -> Vec<u8> {
        let mut info_fields: Vec<u8> = Vec::new();
        for info_field in self.vcf_config.info.iter() {
            let info_field_str = &mut format!(
                "##INFO=<ID=\"{}\",Description=\"{}\"",
                info_field.id, info_field.description,
            );
            let mut additional_fields = String::new();
            for (name, value) in info_field.other.iter() {
                additional_fields.push_str(format!(",{}=\"{}\"", name, value).as_str());
            }
            additional_fields.push_str(">\n");

            info!("Appending {:?} to file", info_field_str);
            let info_field_slice = &mut info_field_str.as_bytes().to_vec();
            let additional_fields_slice = &mut additional_fields.as_bytes().to_vec();
            info_fields.append(info_field_slice);
            info_fields.append(additional_fields_slice);
        }
        info_fields
    }

    pub fn get_format_field(&self) -> Vec<u8> {
        let mut format_fields: Vec<u8> = Vec::new();
        for format_field in self.vcf_config.format.iter() {
            let format_field_str = &mut format!(
                "##FORMAT=<ID=\"{}\",Description=\"{}\"",
                format_field.id, format_field.description
            );
            let mut additional_fields = String::new();
            for (name, value) in format_field.other.iter() {
                additional_fields.push_str(format!(",{}=\"{}\"", name, value).as_str());
            }
            additional_fields.push_str(">\n");

            info!("Appending {:?} to file", format_field_str);
            let format_field_slice = &mut format_field_str.as_bytes().to_vec();
            let additional_fields_slice = &mut additional_fields.as_bytes().to_vec();
            format_fields.append(format_field_slice);
            format_fields.append(additional_fields_slice);
        }
        format_fields
    }
    pub fn get_filter_field(&self) -> Vec<u8> {
        let mut filter_fields: Vec<u8> = Vec::new();
        for filter_field in self.vcf_config.filter.iter() {
            let filter_field_str = &mut format!(
                "##FILTER=<ID=\"{}\",Description=\"{}\"",
                filter_field.id, filter_field.description
            );
            let mut additional_fields = String::new();
            for (name, value) in filter_field.other.iter() {
                additional_fields.push_str(format!(",{}=\"{}\"", name, value).as_str());
            }
            additional_fields.push_str(">\n");

            info!("Appending {:?} to file", filter_field_str);
            let filter_field_slice = &mut filter_field_str.as_bytes().to_vec();
            let additional_fields_slice = &mut additional_fields.as_bytes().to_vec();
            filter_fields.append(filter_field_slice);
            filter_fields.append(additional_fields_slice);
        }
        filter_fields
    }
}

pub trait Validate {
    fn is_valid<T>(&self, field: T) -> bool;
    fn validate<T>(&self, field: T) -> T;
}
