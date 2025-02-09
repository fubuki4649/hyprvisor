mod versions;
mod subcmd;
mod repo_functions;

use structopt::StructOpt;
use crate::subcmd::{install, ls, uninstall};

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
    Uninstall {
        version: String,
    },
}


fn main() {

    let args = Cli::from_args();
    
    match args.cmd {
        Commands::Ls => ls(),
        Commands::Install { version } => install(version, true),
        Commands::Uninstall { version } => uninstall(version),
    }.unwrap()
    
}
