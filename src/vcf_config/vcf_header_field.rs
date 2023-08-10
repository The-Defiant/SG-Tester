use std::collections::HashMap;

pub trait VcfHeaderField {
    fn id(&self) -> String;
    fn description(&self) -> String;
    fn other(&self) -> HashMap<String, String>;
    fn number(&self) -> u8 {
        0
    }
    fn value_type(&self) -> String {
        String::from("string")
    }
    fn get_default_fields(&self) -> Vec<u8>;
    fn get_additional_fields(&self) -> Vec<u8> {
        let mut additional_fields = String::new();
        for (name, value) in self.other().iter() {
            additional_fields.push_str(format!(",{}=\"{}\"", name, value).as_str());
        }
        additional_fields.push_str(">\n");
        additional_fields.as_bytes().to_vec()
    }
    fn get_header_row(&self) -> Vec<u8> {
        let mut header_row: Vec<u8> = Vec::new();
        let default_fields: &mut Vec<u8> = &mut self.get_default_fields();
        let additional_fields: &mut Vec<u8> = &mut self.get_additional_fields();
        header_row.append(default_fields);
        header_row.append(additional_fields);
        header_row
    }
}
