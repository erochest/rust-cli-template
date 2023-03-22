
use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;
use structopt::StructOpt;

mod error;

use error::Result;

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::from_args();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    println!("{:?}", args);

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbosity,
}
