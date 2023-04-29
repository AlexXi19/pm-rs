mod create;

use create::*;
use std::collections::HashMap;
use structopt::StructOpt;

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
    }
}


struct Config {
    services: HashMap<String, u32>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::from_args() {
        Cli::Create { name, command } => {
            let op = CreateOp { name, command };
            op.run()?;
        }
    }

    Ok(())
}
