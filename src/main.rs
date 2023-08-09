use inquire::InquireError;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os;

fn main() -> Result<(), Error> {
    let command_options = CommandOptions::to_vec();
    let command = inquire::Select::new(
        "Which action do you want to perform?",
        command_options,
    )
    .prompt()?;

    let command = CommandOptions::from_str(&command).expect("Valid command value expected.");

    let status = match command {
        CommandOptions::Abort => os::Command::Abort.execute(),
        CommandOptions::Restart => os::Command::Restart(get_delay()?).execute(),
        CommandOptions::Shutdown => os::Command::Shutdown(get_delay()?).execute(),
    }?;

    if status.success() {
        println!("Successfully executed.");
    } else {
        println!("A problem occured during execution: {:?}", status.code());
    }

    Ok(())
}

fn get_delay() -> Result<u64, Error> {
    let delay_options = DelayOptions::to_vec();
    let delay = inquire::Select::new(
        "How long do you want to wait until shutdown?",
        delay_options,
    )
    .prompt()?;

    let delay = DelayOptions::from_str(&delay)
        .expect("Valid delay expected.")
        .as_seconds();
    Ok(delay)
}

enum CommandOptions {
    Abort,
    Restart,
    Shutdown,
}

enum DelayOptions {
    Min5,
    Min10,
    Min15,
    Min30,
    Hour1,
    Hour2,
    Hour6,
    Hour12,
    Day1,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Inquire(InquireError),
}

impl From<InquireError> for Error {
    fn from(value: InquireError) -> Self {
        Error::Inquire(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl CommandOptions {
    fn to_vec() -> Vec<String> {
        vec![
            "Abort".to_owned(),
            "Restart".to_owned(),
            "Shutdown".to_owned(),
        ]
    }

    fn from_str(str: &str) -> Option<CommandOptions> {
        match str {
            "Abort" => Some(CommandOptions::Abort),
            "Restart" => Some(CommandOptions::Restart),
            "Shutdown" => Some(CommandOptions::Shutdown),
            _ => None,
        }
    }
}

impl DelayOptions {
    fn to_vec() -> Vec<String> {
        vec![
            "5 min".to_owned(),
            "10 min".to_owned(),
            "15 min".to_owned(),
            "30 min".to_owned(),
            "1 h".to_owned(),
            "2 h".to_owned(),
            "6 h".to_owned(),
            "12 h".to_owned(),
            "1 d".to_owned(),
        ]
    }

    fn from_str(str: &str) -> Option<DelayOptions> {
        match str {
            "5 min" => Some(DelayOptions::Min5),
            "10 min" => Some(DelayOptions::Min10),
            "15 min" => Some(DelayOptions::Min15),
            "30 min" => Some(DelayOptions::Min30),
            "1 h" => Some(DelayOptions::Hour1),
            "2 h" => Some(DelayOptions::Hour2),
            "6 h" => Some(DelayOptions::Hour6),
            "12 h" => Some(DelayOptions::Hour12),
            "1 d" => Some(DelayOptions::Day1),
            _ => None,
        }
    }

    fn as_seconds(self) -> u64 {
        match self {
            DelayOptions::Min5 => 300,
            DelayOptions::Min10 => 600,
            DelayOptions::Min15 => 900,
            DelayOptions::Min30 => 1800,
            DelayOptions::Hour1 => 3600,
            DelayOptions::Hour2 => 7200,
            DelayOptions::Hour6 => 21600,
            DelayOptions::Hour12 => 43200,
            DelayOptions::Day1 => 86400,
        }
    }
}
