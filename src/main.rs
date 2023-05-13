mod ops;
mod process;
mod utils;

use ops::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cli",
    about = "A rust process manageCli::List()eate and manage daemon processes"
)]

enum Cli {
    #[structopt(name = "create")]
    Create {
        #[structopt(name = "name")]
        name: String,
        #[structopt(name = "command")]
        command: String,
        #[structopt(long = "override")]
        override_flag: bool,
    },
    #[structopt(aliases = &["rm"])]
    Remove {
        #[structopt(name = "name")]
        name: String,
    },
    List {},
    Start {
        #[structopt(name = "name")]
        name: String,
    },
    Stop {
        #[structopt(name = "name")]
        name: String,
    },
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
