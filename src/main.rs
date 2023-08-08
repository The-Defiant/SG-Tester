use clap::Parser;
use log::info;
use std::env;

mod arg_getters;
mod boundaries;
mod cli;
mod command_dispatcher;
mod spec_parser;
mod specs;
mod vcf_config;
mod vcf_parser;
mod write_planner;

fn main() -> Result<(), ()> {
    env_logger::init();
    // https://stackoverflow.com/questions/50775023/why-do-i-get-an-error-when-pattern-matching-a-struct-like-enum-variant-with-fiel
    let cmd: Vec<String> = env::args().collect();
    info!("Running with {:?}.", cmd.join(" "));
    let args: cli::StructuredGeneCLI = cli::StructuredGeneCLI::parse();

    let mode: &str = command_dispatcher::extract_command(&args);
    info!("Running in {} mode.", mode);

    let _result = command_dispatcher::command_dispatcher(&args);
    Ok(())
}
