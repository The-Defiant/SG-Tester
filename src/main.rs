use clap::{Parser, Subcommand, ValueEnum};
use log::info;
use std::env;
use std::fs::File;
use std::path::PathBuf;

mod boundaries;
mod vcf_parser;
mod write_planner;

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
    Generate {
        #[arg()]
        file: PathBuf,
        /// Write n number of variants
        #[arg(short, long, default_value = "1")]
        number_of_variants: Option<i32>,
        /// Write only vcf header
        #[arg(short, long, default_value = "true")]
        only_header: bool,
        #[arg(long, default_value = "ensembl")]
        chrom_set: Option<ChromSet>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum ChromSet {
    Ensembl,
    Ucsc,
}

fn main() -> Result<(), ()> {
    env_logger::init();
    // https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
    let cmd: Vec<String> = env::args().collect();
    info!("Running with {:?}.", cmd.join(" "));
    let args: StructuredGeneCLI = StructuredGeneCLI::parse();
    let mode: &str = extract_command(&args);
    let _file_format = vcf_parser::VcfParser::get_file_format();
    info!("We are running in {} mode.", mode);
    let _result = command_dispatcher(&args);
    Ok(())
}

fn extract_command(args: &StructuredGeneCLI) -> &str {
    match &args.command {
        StructuredGeneCLICommands::Watch {} => "watch",
        StructuredGeneCLICommands::Generate { .. } => "generate",
    }
}

fn command_dispatcher(args: &StructuredGeneCLI) -> Result<(), ()> {
    match &args.command {
        StructuredGeneCLICommands::Watch {} => {
            // create a server
            Ok(())
        }
        StructuredGeneCLICommands::Generate {
            file, chrom_set, ..
        } => {
            // write to a file
            let chromosomes = match chrom_set {
                Some(ChromSet::Ensembl) => boundaries::ENSEMBL_CHROMOSOMES,
                Some(ChromSet::Ucsc) => boundaries::UCSC_CHROMOSOMES,
                None => boundaries::ENSEMBL_CHROMOSOMES,
            };
            info!("Using chrom set from {:?}.", chrom_set.clone().unwrap());
            info!("Using chromosomes: {:?}.", chromosomes);
            let new_file = File::create(file).unwrap();
            let s = String::from("This is length of the variant");
            let _new_description = InfoField::parse_description(s);
            write_planner::WritePlanner::new()
                .add(vcf_parser::VcfParser::get_file_format())
                .add(b"some path")
                .write(&new_file);
            Ok(())
        }
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

struct InfoField {
    // number: InfoNumberValue,  // Integer or value that describes the number of values that INFO can hold
    // r#type: InfoFieldType, //
    // description: String,
    // source: String,
    // version: String
}

impl InfoField {
    fn parse_description(description: String) -> String {
        let quoted_description = format!("\"{}\"", description);
        quoted_description
    }
}

#[test]
fn test_command_extraction() {
    let mut test_args = StructuredGeneCLI {
        command: StructuredGeneCLICommands::Watch {},
    };
    assert_eq!(extract_command(&test_args), "watch");
    test_args.command = StructuredGeneCLICommands::Generate {
        file: PathBuf::from("/tmp/path"),
        chrom_set: None,
        number_of_variants: Some(1),
        only_header: false,
    };
    assert_eq!(extract_command(&test_args), "generate");
}

#[test]
fn string_is_quoted() {
    let s: String = String::from("Dummy");
    let result = InfoField::parse_description(s);
    assert_eq!(String::from("\"Dummy\""), result)
}
