use crate::arg_getters;
use crate::cli;
use crate::vcf_parser::VcfParser;
use crate::write_planner;

pub fn extract_command(args: &cli::StructuredGeneCLI) -> &str {
    match &args.command {
        cli::StructuredGeneCLICommands::Watch {} => "watch",
        cli::StructuredGeneCLICommands::Generate { .. } => "generate",
    }
}

pub fn command_dispatcher(args: &cli::StructuredGeneCLI) -> Result<(), ()> {
    match &args.command {
        cli::StructuredGeneCLICommands::Watch {} => {
            // create a server
            Ok(())
        }
        cli::StructuredGeneCLICommands::Generate {
            output_file,
            chrom_set,
            number_of_variants,
            samples,
            config_file,
            ..
        } => {
            let _chromosomes = arg_getters::choose_chrom_set(chrom_set);
            let _number_of_variants = arg_getters::get_number_of_variants(number_of_variants);
            let _samples = arg_getters::get_samples(samples);
            let _config = arg_getters::get_config_from_file(config_file);

            let output_file = arg_getters::get_output_file(output_file);
            write_planner::WritePlanner::new()
                .add(VcfParser::get_file_format())
                .add(b"some path")
                .write(&output_file);
            Ok(())
        }
    }
}
