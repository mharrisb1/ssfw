use log::info;
use std::path::PathBuf;
use std::process::Child;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use serde::Serialize;

mod errors;
mod poll_watcher;
mod renderer;
mod result;
mod subprocess;
mod watcher;

use crate::errors::SsfwError;
use crate::poll_watcher::PollWatcher;
use crate::renderer::render_command;
use crate::subprocess::execute_command;
use crate::watcher::Watcher;

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
    /// Filter pattern
    #[arg(short, long)]
    pattern: String,

    /// Run command
    #[arg(short, long, default_value = ":")]
    command: String,

    /// Optional working directory
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,

    /// Shell to execute command in
    #[arg(long, default_value = "zsh")]
    sh: Shell,

    /// Optional debounce window (ms)
    #[arg(long, default_value = "500")]
    debounce: u64,

    /// Force poll watcher
    #[arg(long, default_value = "false")]
    force_poll: bool,

    /// Polling interval (ms). Ignored unless force poll is used
    #[arg(long, default_value = "500")]
    poll: u64,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() -> Result<(), SsfwError> {
    let config = Config::parse();
    setup_logging(&config.verbose);
    info!("🪬 ssfw started");
    info!("Command:\t{}", &config.command);
    info!("Pattern:\t{}", &config.pattern);
    let pattern = glob::Pattern::new(&config.pattern)?;
    let f = |path: &_| {
        let mut child: Option<Child> = None;
        let shell: String = config.sh.clone().into();
        let rendered_cmd = render_command(&config.command, path)?;
        execute_command(&rendered_cmd, &shell, &mut child)?;
        Ok(())
    };
    match config.force_poll {
        false => Watcher::new(config.working_dir.as_path()).watch(pattern, config.debounce, f)?,
        true => PollWatcher::new(config.working_dir.as_path()).watch(pattern, config.poll, f)?,
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
