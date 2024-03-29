use log::error;
use regex::{Captures, Error, Regex};
use std::path::Path;

pub(crate) fn render_command(cmd_template: &str, path: &Path) -> Result<String, Error> {
    if let Some(path_str) = path.to_str() {
        let re = Regex::new(r"\{\s*(?P<path>path)\s*\}")?;
        let replacement = |caps: &Captures| -> Result<String, &'static str> {
            match caps.name("path") {
                Some(_) => Ok(path_str.to_string()),
                None => Ok("".to_string()),
            }
        };
        match replace_all(&re, cmd_template, replacement) {
            Ok(s) => return Ok(s),
            Err(e) => {
                error!("Error during variable injection for path: {}", e);
                return Ok(cmd_template.to_string());
            }
        }
    }

    Ok(cmd_template.to_string())
}

fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}
