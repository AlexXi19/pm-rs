use crate::{process::*, utils::*};
use anyhow::Result;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use std::process::{Command, Stdio};

pub fn create(name: String, command_string: String, override_flag: bool) -> Result<()> {

    let mut processes = read_process_file()?;

    for process in &processes.processes {
        if process.name == name {
            if !override_flag {
                return Ok(());
            } else {
                stop_process(process.pid)?;
            }
        }
    }

    // Execute
    let pid = start_process(&command_string)?;
    let start_time = chrono::Local::now().to_rfc3339();

    let process = ManagedProcess {
        name: name.clone(),
        command: command_string.clone(),
        start_time: Some(start_time.clone()),
        pid,
    };

    processes.processes.push(process);
    write_process_file(processes)?;
    Ok(())
}

pub fn list() -> Result<()> {
    let process_details = sync_processes()?;
    let table = process_details
        .iter()
        .map(|process| {
            let status = match process.status {
                ProcessStatus::Active => "Active",
                ProcessStatus::Inactive => "Inactive",
            };
            vec![
                process.name.clone(),
                process.pid.to_string(),
                status.to_string(),
                process.uptime.clone(),
                process.command.clone(),
            ]
        })
        .collect::<Vec<Vec<String>>>()
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "PID".cell().bold(true),
            "Status".cell().bold(true),
            "Uptime".cell().bold(true),
            "Command".cell().bold(true),
        ])
        .bold(false);

    print_stdout(table)?;

    Ok(())
}
