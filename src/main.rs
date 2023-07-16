use clap::{Parser, Subcommand, ValueEnum};
use log::info;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StructuredGeneCLI {
    #[command(subcommand)]
    command: StructuredGeneCLICommands,
}
const ENSEMBL_CHROMOSOMES: [&str; 24] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", 
    "10", "11", "12", "13", "14", "15", "16", "17", 
    "18", "19", "20", "21", "X", "Y", "M"
];
const UCSC_CHROMOSOMES: [&str; 24] = [
    "chr1", "chr2", "chr3", "chr4", "chr5", "chr6", "chr7", "chr8", "chr9", 
    "chr10", "chr11", "chr12", "chr13", "chr14", "chr15", "chr16", "chr17", 
    "chr18", "chr19", "chr20", "chr21", "chrX", "chrY", "chrM"
];


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
        chrom_set: Option<ChromSet>
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum ChromSet {
    Ensembl,
    Ucsc
}

fn main() -> Result<(), ()> {
    env_logger::init();
    // https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
    let cmd: Vec<String> = env::args().collect();
    info!("Running with {:?}.", cmd.join(" "));
    let args: StructuredGeneCLI = StructuredGeneCLI::parse();
    let mode: &str = extract_command(&args);
    let _file_format = VcfParser::get_file_format();
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
        StructuredGeneCLICommands::Generate { file, chrom_set, .. } => {
            // write to a file
            let chromosomes = match chrom_set {
                Some(ChromSet::Ensembl) => ENSEMBL_CHROMOSOMES,
                Some(ChromSet::Ucsc) => UCSC_CHROMOSOMES,
                None => ENSEMBL_CHROMOSOMES
            };
            info!("Using chrom set from {:?}.", chrom_set.clone().unwrap());
            info!("Using chromosomes: {:?}.", chromosomes);
            let new_file = File::create(file).unwrap();


            WritePlanner::new()
                .add(VcfParser::get_file_format())
                .add(b"some path")
                .write(&new_file);
            Ok(())
        }
    }
}

struct VcfParser {}

impl<'a> VcfParser {
    fn get_file_format() -> &'a [u8] {
        b"##fileformat=VCFv4.2"
    }
}

struct WritePlanner<'a> {
    queue: VecDeque<&'a [u8]>,
}

impl<'a> WritePlanner<'a> {
    pub fn new() -> Self {
        let vec_queue: VecDeque<&'a [u8]> = VecDeque::new();
        Self { queue: vec_queue }
    }
    pub fn add(&mut self, text: &'a [u8]) -> &mut Self {
        self.queue.push_back(text);
        self
    }
    pub fn write<T: Write>(&mut self, writable: T) {
        let mut output = BufWriter::new(writable);

        let mut can_pop: bool = !self.queue.is_empty();
        while can_pop {
            can_pop = match self.queue.pop_front() {
                Some(arr) => {
                    let _ = output.write_all(arr);
                    let _ = output.write_all(b"\n");
                    true
                }
                None => false,
            };
        }
        output.flush().unwrap()
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
        only_header: false
    };
    assert_eq!(extract_command(&test_args), "generate");
}
#[test]
fn header_is_created() {
    let expected_header = b"##fileformat=VCFv4.2";
    assert_eq!(VcfParser::get_file_format(), expected_header);
}
#[test]
fn write_planner_adds_to_queue() {
    let mut write_planner = WritePlanner::new();
    let test_value = b"some_binary";
    write_planner.add(test_value);
    write_planner.add(test_value);

    for value in write_planner.queue.iter() {
        assert_eq!(*value, test_value);
    }
}
#[test]
fn write_planner_writes_to_file() {
    use std::io::{Read, Seek, SeekFrom};
    use tempfile::tempfile;

    let mut write_planner = WritePlanner::new();
    let write_planner = write_planner.add(b"Some value").add(b"Another value");

    let mut tmpfile = tempfile().unwrap();
    write_planner.write(&tmpfile);
    // reset the cursor
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut contents = String::new();
    let _ = tmpfile.read_to_string(&mut contents);
    assert_eq!(contents, "Some value\nAnother value\n");
}


