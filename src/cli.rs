use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::boundaries;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct StructuredGeneCLI {
    #[command(subcommand)]
    pub command: StructuredGeneCLICommands,
}
#[derive(Subcommand, Debug)]
pub enum StructuredGeneCLICommands {
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
