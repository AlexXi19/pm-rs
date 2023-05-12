mod utils; 
mod ops;
mod process;

use std::collections::HashMap;
use structopt::StructOpt;
use ops::*;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cli",
    about = "A rust process manager to run create and manage daemon processes"
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
    List {},
    Start {},
    Stop {},
}


struct Config {
    services: HashMap<String, u32>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::from_args() {
        Cli::Create { name, command, override_flag } => {
            create(name, command, override_flag)?;
        }
        Cli::List { } => {
            list()?;
        }
        Cli::Start { } => {
            println!("Start");
        }
        Cli::Stop { } => {
            println!("Stop");
        }
    }

    Ok(())
}
