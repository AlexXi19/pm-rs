use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedProcesses {
    pub processes: Vec<ManagedProcess>,
}

impl Default for ManagedProcesses {
    fn default() -> Self {
        ManagedProcesses {
            processes: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedProcess {
    pub name: String,
    pub command: String,
    pub start_time: Option<String>,
    pub pid: u32,
}

pub fn config_dir() -> Result<String> {
    let home_dir = dirs::home_dir().context("Could not find home directory")?;
    Ok(home_dir
        .join(".config/pm-rs")
        .to_str()
        .context("Unable to convert path to string")?
        .to_string())
}

pub fn config_file() -> Result<String> {
    let config_path = config_dir()?;
    Ok(format!("{}/processes.json", config_path))
}

pub fn create_process_file_if_not_exists() -> Result<()> {
    let config_dir = config_dir()?;
    fs::create_dir_all(&config_dir)?;

    let config_file = config_file()?;

    // Create the file if it doesn't exist or is empty
    if !Path::new(&config_file).exists() || fs::metadata(&config_file)?.len() == 0 {
        let mut file = File::create(config_file)?;
        let json = serde_json::to_string(&ManagedProcesses::default())?;
        file.write_all(json.as_bytes())?;
    };

    Ok(())
}

/// Reads all managed processes into a vector of `Process` structs.
pub fn read_process_file() -> Result<ManagedProcesses> {
    create_process_file_if_not_exists()?;
    let config_file = config_file()?;

    let mut file = File::open(config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let processes: ManagedProcesses = serde_json::from_str(&contents)?;
    Ok(processes)
}

/// Writes all managed processes to a the managed process file.
pub fn write_process_file(managed_processes: ManagedProcesses) -> Result<()> {
    create_process_file_if_not_exists()?;
    let config_file = config_file()?;

    let mut file = File::create(config_file)?;
    let json = serde_json::to_string(&managed_processes)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
