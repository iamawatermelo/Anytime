use std::time::SystemTime;

use log::{info, trace};
use clap::{Parser, Subcommand};
use fern::colors::{Color, ColoredLevelConfig};

mod watcher;
mod userconfig;
mod heartbeat;
mod projectconfig;
mod legacy;

#[derive(Parser, Debug)]
#[command(version, about = "Alternative Wakatime tracker tool", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, help = "Verbose output (up to -vv)", action = clap::ArgAction::Count)]
    verbose: u8
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Track your time")]
    Track {}
}

fn main() {
    let cli = Cli::parse();
    
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);
    
    let colors_level = colors_line
        .info(Color::Magenta)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightMagenta);
    
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date} {level} {target}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(match cli.verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            2 => log::LevelFilter::Trace,
            _ => panic!("woah there, that's way too verbose for me (up to -vv supported)")
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    
    trace!("Set up logging");
    trace!("Parsed CLI: {cli:#?}");
    info!("Hello, world! {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}