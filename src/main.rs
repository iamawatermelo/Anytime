use clap::{Parser, Subcommand};

mod userconfig;
mod heartbeat;
mod config;
mod legacy;

#[derive(Parser, Debug)]
#[command(version, about = "Alternative Wakatime tracker tool", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Track your time")]
    Track {
        
    }
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:#?}")
}
