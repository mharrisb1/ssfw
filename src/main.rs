use std::process::{Child, Command, Stdio};
use std::time::Duration;

use clap::Parser;
use glob::glob;
use notify::{Config, PollWatcher, RecursiveMode, Watcher};

/// ssfw - Super simple file watcher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Monitoring path
    #[arg(short, long)]
    path: String,

    /// Command
    #[arg(short, long)]
    command: String,

    /// Toggle verbosity
    #[arg(long, short)]
    verbose: bool,
}

fn main() -> notify::Result<()> {
    let args = Args::parse();
    let (tx, rx) = std::sync::mpsc::channel();
    let conf = Config::default()
        .with_compare_contents(true)
        .with_poll_interval(Duration::from_millis(500));
    let mut watcher = PollWatcher::new(tx, conf)?;
    log!(
        &args.verbose,
        "Watching files at glob pattern {}",
        &args.path
    );
    glob(&args.path)
        .expect("Failed to read glob pattern")
        .for_each(|entry| match entry {
            Ok(path) => {
                if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
                    eprintln!("Error trying to add file {:?} to watcher: {}", path, e);
                }
            }
            Err(e) => eprintln!("Glob error: {}", e),
        });

    let mut child: Option<Child> = None;
    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = run_cmd(&args.command, &mut child, &args.verbose) {
                    eprintln!("Failed to run command {}: {}", &args.command, e);
                }
                log!(
                    &args.verbose,
                    "Event detected for file(s) {}",
                    event
                        .paths
                        .iter()
                        .map(|p| p.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn run_cmd(
    cmd: &str,
    child_process: &mut Option<Child>,
    verbose: &bool,
) -> Result<(), std::io::Error> {
    if let Some(child) = child_process {
        match child.kill() {
            Ok(_) => log!(
                verbose,
                "Shutting down previous process <pid:{}>",
                child.id()
            ),
            Err(e) => eprintln!("Failed to gracefully shutdown previous process {}", e),
        }
        let _ = child.wait();
    }
    let new_child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    *child_process = Some(new_child);
    Ok(())
}

#[macro_export]
macro_rules! log {
    ($verbose:expr, $($arg:tt)*) => {
        if *$verbose {
            println!($($arg)*);
        }
    };
}
