#[derive(Debug)]
pub(crate) enum SsfwError {
    Notify(notify::Error),
    Glob(globset::Error),
    Cmd(std::io::Error),
    Regex(regex::Error),
    StripPath(std::path::StripPrefixError),
}

impl std::fmt::Display for SsfwError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SsfwError::Notify(..) => write!(f, "Error in notify service"),
            SsfwError::Glob(..) => write!(f, "Invalid glob pattern"),
            SsfwError::Cmd(..) => write!(f, "Error running command"),
            SsfwError::Regex(..) => write!(f, "Error during regex operation"),
            SsfwError::StripPath(..) => write!(f, "Error during strip path operation"),
        }
    }
}

impl std::error::Error for SsfwError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SsfwError::Notify(ref e) => Some(e),
            SsfwError::Glob(ref e) => Some(e),
            SsfwError::Cmd(ref e) => Some(e),
            SsfwError::Regex(ref e) => Some(e),
            SsfwError::StripPath(ref e) => Some(e),
        }
    }
}

impl From<notify::Error> for SsfwError {
    fn from(err: notify::Error) -> Self {
        SsfwError::Notify(err)
    }
}

impl From<globset::Error> for SsfwError {
    fn from(err: globset::Error) -> Self {
        SsfwError::Glob(err)
    }
}

impl From<std::io::Error> for SsfwError {
    fn from(err: std::io::Error) -> Self {
        SsfwError::Cmd(err)
    }
}

impl From<regex::Error> for SsfwError {
    fn from(err: regex::Error) -> Self {
        SsfwError::Regex(err)
    }
}

impl From<std::path::StripPrefixError> for SsfwError {
    fn from(err: std::path::StripPrefixError) -> Self {
        SsfwError::StripPath(err)
    }
}
