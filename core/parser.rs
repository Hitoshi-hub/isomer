use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    /// 
    #[command(Subcommand)]
    action: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(short_flag = 'S')]
    Install {
        packages: Vec<String>,
    },
    #[command(short_flag = 'R')]
    Remove {
        packages: Vec<String>,
    },
    #[command(short_flag = 'Q')]
    Query,
}
