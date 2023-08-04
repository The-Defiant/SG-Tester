use log::{error, info};
use std::fs::{create_dir_all, File};

use std::path::PathBuf;

use crate::boundaries::{ChromSet, ENSEMBL_CHROMOSOMES, UCSC_CHROMOSOMES};
use crate::vcf_config::VCFConfig;

pub fn get_config_from_file(config_file: &PathBuf) -> VCFConfig {
    info!(
        "Loading row data from config {}",
        config_file.to_str().unwrap()
    );
    VCFConfig { _file: config_file }
}

pub fn choose_chrom_set(chrom_set: &Option<ChromSet>) -> [&str; 24] {
    let chromosomes = match chrom_set {
        Some(ChromSet::Ensembl) => ENSEMBL_CHROMOSOMES,
        Some(ChromSet::Ucsc) => UCSC_CHROMOSOMES,
        None => ENSEMBL_CHROMOSOMES,
    };

    info!("Using chrom set from {:?}.", chrom_set.as_ref().unwrap());
    info!("Using chromosomes {:?}.", &chromosomes);
    chromosomes
}

pub fn get_number_of_variants(n: &Option<i32>) -> i32 {
    // just ensure that at least singe variant is being created
    let number_of_variants = match n {
        Some(n) => *n,
        None => 1,
    };
    info!("Create {} variants.", number_of_variants);
    number_of_variants
}

pub fn get_samples(samples: &[String]) -> Vec<&str> {
    let samples = samples.iter().map(|s| &**s).collect();
    info!("Using samples {:?}", &samples);
    samples
}

pub fn get_output_file(output_file: &PathBuf) -> File {
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

    File::create(output_file).unwrap()
}
