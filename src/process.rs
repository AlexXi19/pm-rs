use std::process::{Command, Stdio};

use crate::utils::*;
use anyhow::{Context, Result};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use psutil::process::{self, Process};

#[derive(Debug, Clone)]
pub struct ProcessDetail {
    pub name: String,
    pub pid: String,
    pub status: ProcessStatus,
    pub uptime: String,
    pub command: String,
}

#[derive(Debug, Clone)]
pub enum ProcessStatus {
    Active,
    Inactive,
}

pub fn start_process(command_string: &String) -> Result<u32> {
    // Wrap the command to run it in the background and then wait
    // In the Unix world, many processes will "daemonize" themselves.
    // This means they fork off a child process and then the parent process exits.
    // This has the effect of disconnecting the process from the terminal that started it, making it a "background" process or "daemon".
    // One way to handle this would be to wrap your command in a shell script that blocks until the child process exits.
    let wrapped_command_string = format!("trap 'kill $!' SIGTERM; {} & wait $!", command_string);

    // Execute
    let mut command = Command::new("sh");
    let cp = command
        .arg("-c")
        .arg(wrapped_command_string)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to start process");
    // TODO: Redirect stdout and stderr to a file so we can access it later

    Ok(cp.id())
}

pub fn get_process_detail_by_name(name: &String) -> Result<Option<ProcessDetail>> {
    let mut process_details = sync_processes()?;

    let process_detail = match process_details
        .iter_mut()
        .find(|process| process.name == *name)
    {
        Some(process) => process,
        None => {
            return Ok(None);
        }
    };

    Ok(Some(process_detail.clone()))
}

pub fn sync_processes() -> Result<Vec<ProcessDetail>> {
    let managed_processes = read_process_file()?;

    let mut processes: Vec<ProcessDetail> = vec![];
    for process in &managed_processes.processes {
        processes.push(get_process_detail(process));
    }

    Ok(processes)
}

pub fn get_process_from_pid(pid: u32) -> Result<Option<Process>> {
    let all_processes = process::processes()?;

    for process in all_processes {
        let process = process?;
        if process.pid() == pid {
            return Ok(Some(process));
        }
    }

    Ok(None)
}

pub fn stop_process(pid: u32) -> Result<()> {
    let process = get_process_from_pid(pid)?.context("Process not found")?;

    // Sometimes PID may not exist so we ignore
    process.terminate().ok();

    Ok(())
}

fn get_process_status(pid: u32) -> ProcessStatus {
    let process = get_process_from_pid(pid);
    let process = match process {
        Ok(process) => process,
        Err(_) => return ProcessStatus::Inactive,
    };

    match process {
        Some(_) => ProcessStatus::Active,
        None => ProcessStatus::Inactive,
    }
}

fn humanize_rfc3339(time: String) -> String {
    let start_time = chrono::DateTime::parse_from_rfc3339(&time).unwrap();
    let curr_time = chrono::Local::now();
    let duration = curr_time.signed_duration_since(start_time);
    HumanTime::from(duration).to_text_en(Accuracy::Precise, Tense::Past)
}

fn get_time_since(start_time: &Option<String>) -> String {
    match start_time {
        Some(time) => humanize_rfc3339(time.to_string()),
        None => "Unknown".to_string(),
    }
}

fn get_process_detail(process: &ManagedProcess) -> ProcessDetail {
    let pid = process.pid;
    let status = get_process_status(pid);
    let uptime = match status {
        ProcessStatus::Active => get_time_since(&process.start_time),
        ProcessStatus::Inactive => "Inactive".to_string(),
    };

    let pid = match status {
        ProcessStatus::Active => pid.to_string(),
        ProcessStatus::Inactive => "Inactive".to_string(),
    };

    let command = process.command.clone();

    ProcessDetail {
        name: process.name.clone(),
        pid,
        status,
        uptime,
        command,
    }
}
