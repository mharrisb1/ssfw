use globset::GlobMatcher;
use log::{debug, error, info, trace};
use notify_debouncer_mini::{new_debouncer, notify};
use std::{path::Path, sync::mpsc::channel, time::Duration};

pub(crate) struct Watcher<'a> {
    root: &'a Path,
}

impl<'a> Watcher<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self { root }
    }

    pub fn watch(
        &self,
        pattern: GlobMatcher,
        debounce_ms: u64,
        f: impl Fn(&Path) -> crate::result::Result<()>,
    ) -> crate::result::Result<()> {
        debug!("Using debuounced event watcher");
        let (sender, receiver) = channel();
        let mut debouncer = new_debouncer(Duration::from_millis(debounce_ms), sender)?;
        debouncer
            .watcher()
            .watch(self.root, notify::RecursiveMode::Recursive)?;
        let cwd = std::env::current_dir().unwrap();
        for res in receiver {
            match res {
                Ok(events) => {
                    for event in events {
                        trace!("{:?}", &event);
                        let path = event.path.as_path();
                        let relative_path = path.strip_prefix(&cwd)?;
                        if pattern.is_match(relative_path) {
                            let str_path = relative_path.to_str().unwrap_or_default();
                            info!("Event detected for path {}", str_path);
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
