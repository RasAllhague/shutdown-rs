use std::process::ExitStatus;

use crate::Error;

pub enum Command {
    Abort,
    Restart(u64),
    Shutdown(u64),
}

impl Command {
    pub fn execute(&self) -> Result<ExitStatus, Error> {
        match self {
            Self::Abort => Self::abort(),
            Self::Restart(delay) => Self::restart(*delay),
            Self::Shutdown(delay) => Self::shutdown(*delay),
        }
    }

    fn abort() -> Result<ExitStatus, Error> {
        let output = std::process::Command::new("shutdown").arg("-c").output()?;

        Ok(output.status)
    }

    fn restart(delay: u64) -> Result<ExitStatus, Error> {
        let output = std::process::Command::new("shutdown")
            .arg("--reboot")
            .arg(format!("{delay}"))
            .output()?;

        Ok(output.status)
    }

    fn shutdown(delay: u64) -> Result<ExitStatus, Error> {
        let output = std::process::Command::new("shutdown")
            .arg("--poweroff")
            .arg(format!("{delay}"))
            .output()?;

        Ok(output.status)
    }
}
