use log::{debug, error, info};
use notify::{Config, EventKind, PollWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};

pub(crate) struct Watcher {
    root: PathBuf,
    poll: u64,
}

impl Watcher {
    pub fn new(root: &Option<PathBuf>) -> Self {
        let cwd = std::env::current_dir().unwrap();
        Self {
            root: root.clone().unwrap_or(cwd),
            poll: 500,
        }
    }

    pub fn poll_interval(&mut self, poll_ms: u64) -> &Self {
        self.poll = poll_ms;
        self
    }

    pub fn watch(
        &self,
        pattern: glob::Pattern,
        f: impl Fn(&Path) -> crate::result::Result<()>,
    ) -> crate::result::Result<()> {
        let (sender, receiver) = channel();
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_millis(self.poll));
        let mut notify_watcher = PollWatcher::new(sender, config)?;
        notify_watcher.watch(self.root.as_path(), RecursiveMode::Recursive)?;
        for res in receiver {
            match res {
                Ok(event) => {
                    debug!("{:?}", &event);
                    let paths: Vec<&Path> = event.paths.iter().map(|pb| pb.as_path()).collect();
                    let event_type = match event.kind {
                        EventKind::Create(_) => "CREATE",
                        EventKind::Modify(_) => "UPDATE",
                        EventKind::Remove(_) => "DELETE",
                        _ => "UNKNOWN",
                    };
                    for path in paths {
                        let relative_path = path.strip_prefix(&self.root)?;
                        if pattern.matches_path(relative_path) {
                            info!(
                                "Event type {} detected for path {}",
                                event_type,
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
