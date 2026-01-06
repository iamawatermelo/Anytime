use std::{env::current_dir, path::PathBuf, time::SystemTime};

use anyhow::{Context, Result};
use log::{debug, info, trace};
use clap::{Parser, Subcommand, Args};
use fern::colors::{Color, ColoredLevelConfig};

use crate::{config::{ProjectConfig, UserConfig}, git::GitCli, watcher::FileWatcher};

mod git;
mod watcher;
mod config;
mod heartbeat;

#[derive(Parser, Debug)]
#[command(version, about = "Alternative Wakatime tracker tool", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, help = "Verbose output (up to -vv)", action = clap::ArgAction::Count)]
    verbose: u8
}

#[derive(Args, Debug)]
struct TrackArgs {
    project_path: Option<PathBuf>
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Track your time")]
    Track {
        #[clap(flatten)]
        args: TrackArgs
    },
    
    #[command(about = "Track your time")]
    Test {
        #[clap(flatten)]
        args: TrackArgs
    }
}

fn main() -> Result<()> {
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
    info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    
    let user_config = UserConfig::load()?;
    debug!("loaded user cfg: {user_config:#?}");
    
    let git_cli = GitCli::new().ok();
    match git_cli {
        Some(ref c) => debug!("using git v{}.{}.{}", c.version_tuple.0, c.version_tuple.1, c.version_tuple.2),
        None => debug!("no git cli found")
    }
    
    match cli.command {
        Commands::Track { args } => {
            let project_path = args.project_path
                .or(git_cli.and_then(|c| {
                    let root = c.get_project_root();
                    trace!("git cli root {root:#?}");
                    
                    root.ok()
                }))
                .or(current_dir().ok())
                .context("couldn't determine project path")?;
            debug!("using project path {project_path:#?}");
            
            let config = ProjectConfig::load(&project_path, &user_config)?;
            debug!("using project config {config:#?}");
            
            let watcher = FileWatcher::new(project_path.as_path(), &config);
        }
        Commands::Test { .. } => todo!()
    }
    
    Ok(())
}