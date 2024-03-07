use crate::errors::SsfwError;

pub(crate) type Result<T> = std::result::Result<T, SsfwError>;
