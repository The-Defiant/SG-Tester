use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StructuredGeneCLI {
    #[command(subcommand)]
    command: StructuredGeneCLICommands,
}

#[derive(Subcommand, Debug)]
enum StructuredGeneCLICommands {
    /// Create an API to fetch vcf content
    Watch {},
    /// Generate a single vcf content
    Generate {},
}

fn main() -> Result<(), ()> {
    env_logger::init();
    // https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
    let args: StructuredGeneCLI = StructuredGeneCLI::parse();
    let mode: &str = extract_command(&args);
    let _file_format = VcfParser::get_file_format();
    info!("We are running in {}", mode);
    Ok(())
}

fn extract_command(args: &StructuredGeneCLI) -> &str {
    match args.command {
        StructuredGeneCLICommands::Watch {} => "watch",
        StructuredGeneCLICommands::Generate {} => "generate",
    }
}

#[derive(Debug)]
struct VcfParser {
}

impl VcfParser {
    fn get_file_format() -> &'static[u8] {
        b"##fileformat=VCFv4.2"
    }
}

#[test]
fn test_command_extraction() {
    let mut test_args = StructuredGeneCLI {
        command: StructuredGeneCLICommands::Watch {},
    };
    assert_eq!(extract_command(&test_args), "watch");
    test_args.command = StructuredGeneCLICommands::Generate {};
    assert_eq!(extract_command(&test_args), "generate");
}
#[test]
fn header_is_created() {
    let expected_header = b"##fileformat=VCFv4.2";
    assert_eq!(VcfParser::get_file_format(), expected_header);
}

