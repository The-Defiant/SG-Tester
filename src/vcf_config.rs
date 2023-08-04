use std::path::PathBuf;

#[derive(Debug)]
pub struct VCFConfig<'a> {
    pub _file: &'a PathBuf,
}
