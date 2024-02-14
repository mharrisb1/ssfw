#[derive(Debug)]
pub(crate) enum SsfwError {
    Glob(glob::GlobError),
    Notify(notify::Error),
    Pattern(glob::PatternError),
    Cmd(std::io::Error),
    EmptyFileSet,
}

impl std::fmt::Display for SsfwError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SsfwError::Notify(..) => write!(f, "Error in notify service"),
            SsfwError::Pattern(..) => write!(f, "Invalid glob pattern"),
            SsfwError::Glob(..) => write!(f, "Glob error"),
            SsfwError::Cmd(..) => write!(f, "Error running command"),
            SsfwError::EmptyFileSet => write!(f, "Pattern did not match any files"),
        }
    }
}

impl std::error::Error for SsfwError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SsfwError::Notify(ref e) => Some(e),
            SsfwError::Pattern(ref e) => Some(e),
            SsfwError::Glob(ref e) => Some(e),
            SsfwError::Cmd(ref e) => Some(e),
            SsfwError::EmptyFileSet => None,
        }
    }
}

impl From<notify::Error> for SsfwError {
    fn from(err: notify::Error) -> Self {
        SsfwError::Notify(err)
    }
}

impl From<glob::PatternError> for SsfwError {
    fn from(err: glob::PatternError) -> Self {
        SsfwError::Pattern(err)
    }
}

impl From<glob::GlobError> for SsfwError {
    fn from(err: glob::GlobError) -> Self {
        SsfwError::Glob(err)
    }
}

impl From<std::io::Error> for SsfwError {
    fn from(err: std::io::Error) -> Self {
        SsfwError::Cmd(err)
    }
}
