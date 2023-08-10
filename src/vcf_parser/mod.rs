use crate::vcf_config::{VcfConfig, VcfHeader};

pub struct VcfParser {
    pub vcf_config: VcfConfig,
}

impl<'a> VcfParser {
    pub fn get_file_format() -> &'a [u8] {
        b"##fileformat=VCFv4.2\n"
    }

    pub fn get_info_fields(&self) -> Vec<u8> {
        self.vcf_config.get_info_rows()
    }
    pub fn get_format_field(&self) -> Vec<u8> {
        self.vcf_config.get_format_rows()
    }
    pub fn get_filter_field(&self) -> Vec<u8> {
        self.vcf_config.get_filter_rows()
    }
}
