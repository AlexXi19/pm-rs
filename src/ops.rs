use crate::{process::*, utils::*};
use anyhow::{Context, Result};
use cli_table::{print_stdout, Cell, Style, Table};

pub fn create(name: String, command_string: String, override_flag: bool) -> Result<()> {
    let mut processes = read_process_file()?;

    for process in &processes.processes {
        if process.name == name {
            if !override_flag {
                println!("Process with name {} already exists", name);
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

pub fn remove(name: String) -> Result<()> {
    let mut processes = read_process_file()?.processes;
    let process_detail = get_process_detail_by_name(&name)?.context("Process not found")?;

    match process_detail.status {
        ProcessStatus::Active => {
            stop_process(process_detail.pid.parse::<u32>()?)?;
        }
        ProcessStatus::Inactive => {}
    }

    processes.retain(|process| process.name != name);
    write_process_file(ManagedProcesses { processes })?;
    Ok(())
}

pub fn list() -> Result<()> {
    let process_details = sync_processes()?;

    if process_details.len() == 0 {
        println!("No processes are being managed");
        return Ok(());
    }

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

pub fn start(name: String) -> Result<()> {
    let mut processes = read_process_file()?.processes;
    let process_detail = get_process_detail_by_name(&name)?.context("Process not found")?;

    processes.remove(processes.iter().position(|p| p.name == name).unwrap());

    let new_process = match process_detail.status {
        ProcessStatus::Active => {
            println!("Process {} is already active", name);
            return Ok(());
        }
        ProcessStatus::Inactive => {
            let pid = start_process(&process_detail.command)?;
            let start_time = chrono::Local::now().to_rfc3339();

            ManagedProcess {
                name: name.clone(),
                command: process_detail.command.clone(),
                start_time: Some(start_time.clone()),
                pid,
            }
        }
    };

    processes.push(new_process);
    write_process_file(ManagedProcesses { processes })?;

    Ok(())
}

pub fn stop(name: String) -> Result<()> {
    let processes = read_process_file().unwrap().processes;
    let process_detail = get_process_detail_by_name(&name)?.context("Process not found")?;

    match process_detail.status {
        ProcessStatus::Active => {
            stop_process(process_detail.pid.parse::<u32>()?)?;
        }
        ProcessStatus::Inactive => {}
    };

    write_process_file(ManagedProcesses { processes }).unwrap();

    Ok(())
}

pub fn restart(name: String) -> Result<()> {
    stop(name.clone())?;
    start(name.clone())?;

    Ok(())
}
