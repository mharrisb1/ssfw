use log::info;
use std::path::PathBuf;
use std::process::Child;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use serde::Serialize;

mod errors;
mod renderer;
mod result;
mod subprocess;
mod watcher;

use crate::errors::SsfwError;
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
    #[arg(short, long)]
    working_dir: Option<PathBuf>,

    /// Shell to execute command in
    #[arg(long, default_value = "zsh")]
    sh: Shell,

    /// Optional debounce window (mulliseconds)
    #[arg(long, default_value = "500")]
    debounce: u64,

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
    Watcher::new(&config.working_dir).watch(pattern, config.debounce, |path| {
        let mut child: Option<Child> = None;
        let shell: String = config.sh.clone().into();
        let rendered_cmd = render_command(&config.command, path)?;
        execute_command(&rendered_cmd, &shell, &mut child)?;
        Ok(())
    })?;
    Ok(())
}

fn setup_logging(verbose: &Verbosity<InfoLevel>) {
    env_logger::Builder::new()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(verbose.log_level_filter())
        .init();
}
