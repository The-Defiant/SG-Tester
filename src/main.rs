use clap::{Parser, Subcommand};
use log::{error, info};
use std::env;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

mod boundaries;
mod spec_parser;
mod specs;
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
        /// configuration to use
        #[arg()]
        config_file: PathBuf,
        /// output file
        #[arg(default_value = "result.vcf")]
        output_file: PathBuf,
        /// Write n number of variants
        #[arg(short, long, default_value = "1")]
        number_of_variants: Option<i32>,
        /// Write only vcf header
        #[arg(short, long, default_value = "true")]
        only_header: bool,
        /// Chromosome set to use
        #[arg(long, default_value = "ensembl")]
        chrom_set: Option<boundaries::ChromSet>,
        /// Samples to use
        #[arg(short = 's', long, default_values_t = vec![String::from("S1")])]
        samples: Vec<String>,
    },
}

fn main() -> Result<(), ()> {
    env_logger::init();
    // https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
    let cmd: Vec<String> = env::args().collect();
    info!("Running with {:?}.", cmd.join(" "));
    let args: StructuredGeneCLI = StructuredGeneCLI::parse();

    let mode: &str = extract_command(&args);
    info!("Running in {} mode.", mode);

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
            output_file,
            chrom_set,
            number_of_variants,
            samples,
            config_file,
            ..
        } => {
            let _chromosomes = boundaries::choose_chrom_set(chrom_set);
            let _number_of_variants = boundaries::get_number_of_variants(number_of_variants);
            let _samples = get_samples(samples);
            let _config = get_config_from_file(config_file);

            let output_file = get_output_file(output_file);
            write_planner::WritePlanner::new()
                .add(vcf_parser::VcfParser::get_file_format())
                .add(b"some path")
                .write(&output_file);
            Ok(())
        }
    }
}

fn get_samples(samples: &Vec<String>) -> Vec<&str> {
    let samples = samples.iter().map(|s| &**s).collect();
    info!("Using samples {:?}", &samples);
    samples
}

fn get_output_file(output_file: &PathBuf) -> File {
    info!("Writing to output file: {}", output_file.to_str().unwrap());
    match output_file.parent() {
        Some(parent_dir) => {
            if !parent_dir.exists() & !parent_dir.to_str().unwrap().is_empty() {
                info!(
                    "Parent dir: {:?} does not exist, generating it.",
                    parent_dir
                );
                match create_dir_all(parent_dir) {
                    Ok(_) => info!("Created {:?}", parent_dir),
                    Err(_) => {
                        error!("Problem creating path to {:?}", output_file);
                        panic!("Exiting")
                    }
                }
            }
        }
        None => {
            error!("Provided root or non existing path {:?}", output_file);
            panic!("Exiting")
        }
    };

    // if !parent_dir.exists() {}
    let new_file = File::create(output_file).unwrap();
    new_file
}

#[derive(Debug)]
struct VCFConfig<'a> {
    _file: &'a PathBuf,
}

fn get_config_from_file(config_file: &PathBuf) -> VCFConfig {
    info!(
        "Loading row data from config {}",
        config_file.to_str().unwrap()
    );
    VCFConfig { _file: config_file }
}
