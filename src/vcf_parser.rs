use log::info;

use crate::vcf_config::VcfConfig;

pub struct VcfParser {
    pub vcf_config: VcfConfig,
}

impl<'a> VcfParser {
    pub fn get_file_format() -> &'a [u8] {
        b"##fileformat=VCFv4.2\n"
    }

    pub fn get_info_fields(self) -> Vec<u8> {
        let mut info_fields: Vec<u8> = Vec::new();
        for info_field in self.vcf_config.info.iter() {
            let info_field_str = &mut format!(
                "##INFO=<ID=\"{}\">,Number=\"{}\",Description=\"{}\"",
                info_field.id, info_field.number, info_field.description,
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
}
