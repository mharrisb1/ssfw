use log::{debug, error, info, trace};
use notify_debouncer_mini::{new_debouncer_opt, notify, Config};
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
        pattern: glob::Pattern,
        debounce_ms: u64,
        f: impl Fn(&Path) -> crate::result::Result<()>,
    ) -> crate::result::Result<()> {
        debug!("Using debuounced event watcher");
        let (sender, receiver) = channel();
        let config = Config::default()
            .with_timeout(Duration::from_millis(debounce_ms))
            .with_notify_config(notify::Config::default().with_compare_contents(true));
        let mut debouncer = new_debouncer_opt::<_, notify::FsEventWatcher>(config, sender)?;
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
