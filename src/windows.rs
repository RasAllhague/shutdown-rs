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
        let output = std::process::Command::new("shutdown")
            .arg("/a")
            .output()?;
        
        Ok(output.status)
    }
    
    fn restart(delay: u64) -> Result<ExitStatus, Error> {
        let output = std::process::Command::new("shutdown")
            .arg("/r")
            .arg(format!("/t {delay}"))
            .output()?;
        
        Ok(output.status)
    }
    
    fn shutdown(delay: u64) -> Result<ExitStatus, Error> {
        let output = std::process::Command::new("shutdown")
            .arg("/s")
            .arg(format!("/t {delay}"))
            .output()?;
        
        Ok(output.status)
    }
}