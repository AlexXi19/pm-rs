mod ops;
mod process;
mod utils;

use ops::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pmrs",
    about = "A rust process manager to start and manage processes"
)]
enum Cli {
    #[structopt(name = "create")]
    /// Creates and starts a new process with name
    Create {
        #[structopt(name = "name")]
        name: String,
        #[structopt(name = "command")]
        command: String,
        #[structopt(long = "override")]
        override_flag: bool,
    },
    #[structopt(aliases = &["rm"])]
    /// Removes an inactive process with name
    Remove {
        #[structopt(name = "name")]
        name: String,
    },
    /// Lists all processes being managed
    List {},
    /// Starts an inactive process with name
    Start {
        #[structopt(name = "name")]
        name: String,
    },
    /// Stops an active process with name
    Stop {
        #[structopt(name = "name")]
        name: String,
    },
    /// Restarts an active process with name
    Restart {
        #[structopt(name = "name")]
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::from_args() {
        Cli::Create {
            name,
            command,
            override_flag,
        } => {
            create(name, command, override_flag)?;
        }
        Cli::Remove { name } => {
            remove(name)?;
        }
        Cli::List {} => {
            list()?;
        }
        Cli::Start { name } => {
            start(name)?;
        }
        Cli::Stop { name } => {
            stop(name)?;
        }
        Cli::Restart { name } => {
            restart(name)?;
        }
    }

    Ok(())
}
