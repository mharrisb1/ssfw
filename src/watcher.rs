use log::{error, info, trace};
// use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};

pub(crate) struct Watcher {
    root: PathBuf,
}

impl Watcher {
    pub fn new(root: &Option<PathBuf>) -> Self {
        let cwd = std::env::current_dir().unwrap();
        Self {
            root: root.clone().unwrap_or(cwd),
        }
    }

    pub fn watch(
        &self,
        pattern: glob::Pattern,
        debounce_ms: u64,
        f: impl Fn(&Path) -> crate::result::Result<()>,
    ) -> crate::result::Result<()> {
        let (sender, receiver) = channel();
        let mut debouncer = new_debouncer(Duration::from_millis(debounce_ms), sender)?;
        debouncer
            .watcher()
            .watch(self.root.as_path(), RecursiveMode::Recursive)?;
        let cwd = std::env::current_dir().unwrap();
        for res in receiver {
            match res {
                Ok(events) => {
                    for event in events {
                        trace!("{:?}", &event);
                        let path = event.path.as_path();
                        let relative_path = path.strip_prefix(&cwd)?;
                        if pattern.matches_path(relative_path) {
                            info!(
                                "Event detected for path {}",
                                relative_path.to_str().unwrap_or_default()
                            );
                            f(relative_path)?;
                        }
                    }
                }
                Err(e) => error!("Watch error: {:?}", e),
            }
        }
        Ok(())
    }
}
