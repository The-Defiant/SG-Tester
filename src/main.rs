use std::io::{stdout, Write};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StructuredGeneCLI {
    #[command(subcommand)]
    command: Option<StructuredGeneCLICommands>,
}

#[derive(Subcommand, Debug)]
enum StructuredGeneCLICommands {
    /// Create an API to fetch vcf content
    Watch {},
    /// Generate a single vcf content
    Generate {},
}


fn main() -> Result<(), ()> {
    // https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
    let args: StructuredGeneCLI = parse_arguments();
    let mode: &str = extract_command(&args);
    let out = stdout();
    let _c = writeln!(&out, "Using mode: {}", mode); 
    Ok(())
}


fn parse_arguments() -> StructuredGeneCLI {
    StructuredGeneCLI::parse()}

fn extract_command(args:  &StructuredGeneCLI) -> &str {
    match args.command.as_ref().unwrap() {
        StructuredGeneCLICommands::Watch {} => "watch",
        StructuredGeneCLICommands::Generate {} => "generate",
    }
}


#[test]
fn test_command_extraction() {
    let mut test_args = StructuredGeneCLI { command : None, };
    test_args.command = Some(StructuredGeneCLICommands::Watch {});
    assert_eq!(extract_command(&test_args), "watch");
    test_args.command = Some(StructuredGeneCLICommands::Generate{});
    assert_eq!(extract_command(&test_args), "generate");
}




