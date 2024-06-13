use std::{path::Path, sync::mpsc::channel, time::Duration};

use globset::GlobMatcher;
use log::{debug, error, info, trace};
use notify::{Config, Watcher as IWatcher};

pub(crate) struct PollWatcher<'a> {
    root: &'a Path,
}

impl<'a> PollWatcher<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self { root }
    }

    pub fn watch(
        &self,
        pattern: GlobMatcher,
        poll_ms: u64,
        f: impl Fn(&Path) -> crate::result::Result<()>,
    ) -> crate::result::Result<()> {
        debug!("Using poll watcher");
        let (sender, receiver) = channel();
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_millis(poll_ms));
        let mut watcher = notify::PollWatcher::new(sender, config)?;
        watcher.watch(self.root, notify::RecursiveMode::Recursive)?;
        for res in receiver {
            match res {
                Ok(event) => {
                    trace!("{:?}", event);
                    for path in event.paths {
                        let path = path.as_path();
                        let relative_path = path;
                        if pattern.is_match(relative_path) {
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
