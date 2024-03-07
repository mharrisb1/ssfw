use log::{debug, error, info};
use std::io::{self, Read};
use std::process::{Child, Command, Stdio};

pub(crate) fn run(cmd: &str, shell: &str) -> io::Result<Child> {
    info!("Running command: {} -c '{}'", shell, &cmd);
    Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub(crate) fn terminate(child: &mut Option<Child>) -> io::Result<()> {
    if let Some(ref mut previous_child) = child {
        debug!("Killing child process: {}", previous_child.id());
        if let Err(e) = previous_child.kill() {
            error!("Failed to kill previous process: {}", e);
        }
        debug!(
            "Waiting for child process to terminate: {}",
            previous_child.id()
        );
        if let Err(e) = previous_child.wait() {
            error!("Failed to wait on previous process: {}", e);
        }
    }
    *child = None;
    Ok(())
}

pub(crate) fn execute_command(cmd: &str, shell: &str, child: &mut Option<Child>) -> io::Result<()> {
    terminate(child)?;
    *child = Some(run(cmd, shell)?);
    if let Some(ref mut proc) = child {
        let exit_status = proc.wait()?;
        if exit_status.success() {
            let mut stdout = String::new();
            if let Some(ref mut stdout_pipe) = proc.stdout {
                stdout_pipe.read_to_string(&mut stdout)?;
            }
            print!("{}", stdout);
        } else {
            let mut stderr = String::new();
            if let Some(ref mut stderr_pipe) = proc.stderr {
                stderr_pipe.read_to_string(&mut stderr)?;
            }
            error!("Error running command");
            eprint!("{}", stderr);
        }
    }
    Ok(())
}
