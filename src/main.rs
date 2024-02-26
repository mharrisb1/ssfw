use std::process::Child;
use std::time::Duration;

use log::{debug, error, info};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use notify::Watcher;
use serde::Serialize;

mod errors;
mod renderer;
mod subprocess;

use crate::errors::SsfwError;
use crate::renderer::render_command;
use crate::subprocess::execute_command;

#[derive(clap::ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Shell {
    Zsh,
    Bash,
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Shell> for String {
    fn from(val: Shell) -> Self {
        match val {
            Shell::Zsh => "zsh".to_string(),
            Shell::Bash => "bash".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    /// Monitoring path/glob
    #[arg(short, long)]
    path: String,

    /// Run command
    #[arg(short, long, default_value = ":")]
    command: String,

    /// Poll duration (ms)
    #[arg(long, default_value_t = 500)]
    poll: u64,

    /// Shell to execute command in
    #[arg(long, default_value = "zsh")]
    sh: Shell,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() -> Result<(), SsfwError> {
    let config = Config::parse();
    setup_logging(&config.verbose);
    info!("🪬 ssfw started");
    info!("On change:\t{}", &config.command);
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = init_watcher(tx, config.poll)?;
    info!("Glob pattern:\t{}", &config.path);
    let mut paths = glob::glob(&config.path)?;
    register_paths(&mut watcher, &mut paths)?;
    let mut child: Option<Child> = None;
    for res in rx {
        match res {
            Ok(event) => {
                debug!(
                    "Event detected for file(s) {}",
                    event
                        .paths
                        .iter()
                        .map(|p| p.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                let shell: String = config.sh.clone().into();
                let cmd = render_command(&config.command, &event)?;
                execute_command(&cmd, &shell, &mut child)?;
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn setup_logging(verbose: &Verbosity<InfoLevel>) {
    env_logger::Builder::new()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(verbose.log_level_filter())
        .init();
}

fn init_watcher<F>(handler: F, poll_ms: u64) -> notify::Result<notify::PollWatcher>
where
    F: notify::EventHandler,
{
    debug!("Initializing PollWatcher with poll_ms={}", poll_ms);
    let config = notify::Config::default()
        .with_compare_contents(true)
        .with_poll_interval(Duration::from_millis(poll_ms));
    notify::PollWatcher::new(handler, config)
}

fn register_paths(
    watcher: &mut notify::PollWatcher,
    paths: &mut glob::Paths,
) -> Result<(), SsfwError> {
    let mut n = 0;
    for entry in paths.into_iter() {
        let path = entry?;
        if let Err(e) = watcher.watch(&path, notify::RecursiveMode::NonRecursive) {
            error!("Error adding {} to watcher: {}", path.display(), e);
        } else {
            debug!("Added {} to watcher", path.display());
            n += 1;
        }
    }
    if n == 0 {
        error!("No files matched given glob pattern");
        Err(SsfwError::EmptyFileSet)
    } else {
        info!("Matching files:\t{}", n);
        Ok(())
    }
}
