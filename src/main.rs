mod versions;
mod subcmd;

use anyhow::Error;
use structopt::StructOpt;
use crate::subcmd::{install, ls};

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Commands,
}

#[derive(StructOpt)]
enum Commands {
    #[structopt(about = "List available Hyprland versions")]
    Ls,
    #[structopt(about = "Adds two numbers")]
    Install {
        version: String,
    },
    #[structopt(about = "Multiplies two numbers")]
    Uninstall,
}

fn uninstall() -> Result<(), Error> {
    Ok(())
}


fn main() {

    let args = Cli::from_args();
    
    match args.cmd {
        Commands::Ls => ls(),
        Commands::Install { version } => install(version, true),
        Commands::Uninstall => uninstall(),
    }.unwrap()
    
}
