pub struct _InfoField {
    // number: InfoNumberValue,  // Integer or value that describes the number of values that INFO can hold
    // r#type: InfoFieldType, //
    // description: String,
    // source: String,
    // version: String
}

impl _InfoField {
    pub fn _parse_description(description: String) -> String {
        let quoted_description = format!("\"{}\"", description);
        quoted_description
    }
}

// struct InfoFieldType <'a> {
// value:  &'a String

// }

// impl InfoFieldType

// enum InfoNumberValue {
//     PerAllele,    // R
//     PerAltAllele, // A
//     PerGenotype,  // G
//     Known,        //i32
//     Unknown,       // .
//     FlagEntry     // 0
// }

#[test]
fn string_is_quoted() {
    let s: String = String::from("Dummy");
    let result = _InfoField::_parse_description(s);
    assert_eq!(String::from("\"Dummy\""), result)
}
