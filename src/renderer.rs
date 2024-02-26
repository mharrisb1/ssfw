use log::{debug, error};
use notify::Event;
use regex::{Captures, Error, Regex};

pub(crate) fn render_command(cmd_template: &str, event_ctx: &Event) -> Result<String, Error> {
    let fnames: Vec<String> = event_ctx
        .paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    if let Some(fname) = fnames.first() {
        debug!("Attempting to render command with variable: fname");
        let re = Regex::new(
            r"(?x)
        \{\s*(?P<fname>fname)\s*\} # file name variables
        ",
        )?;
        let replacement = |caps: &Captures| -> Result<String, &'static str> {
            match caps.name("fname") {
                Some(_) => Ok(fname.clone()),
                None => Ok("".to_string()),
            }
        };
        match replace_all(&re, cmd_template, replacement) {
            Ok(s) => return Ok(s),
            Err(e) => {
                error!("Error during variable injection for fname: {}", e);
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
