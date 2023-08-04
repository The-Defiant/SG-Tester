pub struct VcfParser {}

impl<'a> VcfParser {
    pub fn get_file_format() -> &'a [u8] {
        b"##fileformat=VCFv4.2"
    }
}
//     pub fn get_info_field(
//         id: String,
//         number: String,
//         info_type: String,
//         description: String,
//         version: String
//     ) -> Result<(), ()> {
//
//
//     }
// }
//
#[test]
fn header_is_created() {
    let expected_header = b"##fileformat=VCFv4.2";
    assert_eq!(VcfParser::get_file_format(), expected_header);
}
