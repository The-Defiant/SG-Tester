use clap::ValueEnum;
use log::info;

pub const ENSEMBL_CHROMOSOMES: [&str; 24] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "X", "Y", "M",
];
pub const UCSC_CHROMOSOMES: [&str; 24] = [
    "chr1", "chr2", "chr3", "chr4", "chr5", "chr6", "chr7", "chr8", "chr9", "chr10", "chr11",
    "chr12", "chr13", "chr14", "chr15", "chr16", "chr17", "chr18", "chr19", "chr20", "chr21",
    "chrX", "chrY", "chrM",
];

#[derive(Clone, Debug, ValueEnum)]
pub enum ChromSet {
    Ensembl,
    Ucsc,
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
