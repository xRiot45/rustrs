use crate::cli::{DevArgs, DevCommands, DevLifecycleArgs};
use crate::error::CliError;
use crate::process::runner::{ProcessRunner, SystemProcessRunner};
use crate::process::spec::ProcessSpec;
use crate::process::tools::require_tool;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

static DEV_RUNNING: AtomicBool = AtomicBool::new(true);

pub fn execute(args: &DevArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let root = resolve_root(args.path.clone())?;
    if let Some(command) = &args.command {
        match command {
            DevCommands::Up(a) => dev_up(&root, a, output),
            DevCommands::Down(a) => dev_down(&root, a, output),
            DevCommands::Logs(a) => dev_logs(&root, a, output),
        }
    } else if args.watch {
        dev_watch(&root, output)
    } else {
        dev_run(&root, output)
    }
}

fn resolve_root(path: Option<PathBuf>) -> Result<PathBuf, CliError> {
    let root = path.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let cargo = root.join("Cargo.toml");
    if !cargo.is_file() {
        return Err(CliError::InvalidInput(format!(
            "Cargo.toml not found in {}",
            root.display()
        )));
    }
    Ok(root)
}

fn dev_run(root: &Path, _output: &crate::cli::Output) -> Result<(), CliError> {
    require_tool("cargo").map_err(|_| CliError::MissingTool("cargo".to_string()))?;
    DEV_RUNNING.store(true, Ordering::SeqCst);
    let _ = ctrlc::set_handler(|| {
        DEV_RUNNING.store(false, Ordering::SeqCst);
    });

    let mut spec = ProcessSpec::new("cargo");
    spec.args = vec!["run".to_string()];
    spec.cwd = Some(root.to_path_buf());
    let runner = SystemProcessRunner;
    let mut child = runner
        .spawn(spec)
        .map_err(|error| CliError::ExternalCommand(error.to_string()))?;

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    return Ok(());
                }
                return Err(CliError::ExternalCommand(format!(
                    "application exited with status {}",
                    status.code().unwrap_or(-1)
                )));
            }
            Ok(None) => {
                if !DEV_RUNNING.load(Ordering::SeqCst) {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(CliError::Cancelled);
                }
                thread::sleep(Duration::from_millis(100));
            }
            Err(error) => {
                let _ = child.kill();
                return Err(CliError::ExternalCommand(error.to_string()));
            }
        }
    }
}

fn dev_watch(root: &Path, output: &crate::cli::Output) -> Result<(), CliError> {
    require_tool("cargo").map_err(|_| CliError::MissingTool("cargo".to_string()))?;
    DEV_RUNNING.store(true, Ordering::SeqCst);
    let _ = ctrlc::set_handler(|| {
        DEV_RUNNING.store(false, Ordering::SeqCst);
    });

    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .map_err(|error| CliError::ExternalCommand(error.to_string()))?;

    let watch_target = if root.join("src").is_dir() {
        root.join("src")
    } else {
        root.to_path_buf()
    };
    watcher
        .watch(&watch_target, RecursiveMode::Recursive)
        .map_err(|error| CliError::ExternalCommand(error.to_string()))?;

    while DEV_RUNNING.load(Ordering::SeqCst) {
        if !output.quiet {
            output.info("Starting application...");
        }
        let mut spec = ProcessSpec::new("cargo");
        spec.args = vec!["run".to_string()];
        spec.cwd = Some(root.to_path_buf());
        let runner = SystemProcessRunner;
        let mut child = match runner.spawn(spec) {
            Ok(child) => child,
            Err(error) => {
                output.warn(&format!("failed to start application: {error}"));
                wait_for_change(&rx, Duration::from_secs(2));
                continue;
            }
        };

        let mut dirty = false;
        'running: loop {
            if !DEV_RUNNING.load(Ordering::SeqCst) {
                let _ = child.kill();
                let _ = child.wait();
                return Err(CliError::Cancelled);
            }

            match child.try_wait() {
                Ok(Some(status)) => {
                    if !status.success() {
                        output.warn(&format!(
                            "application exited with status {}",
                            status.code().unwrap_or(-1)
                        ));
                    }
                    wait_for_change(&rx, Duration::from_secs(3600));
                    break 'running;
                }
                Ok(None) => {
                    while let Ok(event) = rx.try_recv() {
                        if let Ok(event) = event {
                            if is_relevant_event(&event) {
                                dirty = true;
                            }
                        }
                    }
                    if dirty {
                        thread::sleep(Duration::from_millis(300));
                        while let Ok(event) = rx.try_recv() {
                            let _ = event;
                        }
                        let _ = child.kill();
                        let _ = child.wait();
                        break 'running;
                    }
                    thread::sleep(Duration::from_millis(100));
                }
                Err(error) => {
                    output.warn(&format!("process error: {error}"));
                    let _ = child.kill();
                    let _ = child.wait();
                    break 'running;
                }
            }
        }
    }

    Ok(())
}

fn is_relevant_event(event: &Event) -> bool {
    event.paths.iter().any(|path| {
        let text = path.to_string_lossy();
        !text.contains("/target/")
            && !text.contains("\\target\\")
            && !text.contains("/.git/")
            && !text.contains("\\.git\\")
    })
}

fn wait_for_change(rx: &mpsc::Receiver<notify::Result<Event>>, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if !DEV_RUNNING.load(Ordering::SeqCst) {
            return;
        }
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(_) => return,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
        }
    }
}

fn state_dir(root: &Path) -> PathBuf {
    root.join(".rustrs").join("dev")
}

fn pid_file(root: &Path) -> PathBuf {
    state_dir(root).join("pid")
}

fn log_file(root: &Path) -> PathBuf {
    state_dir(root).join("dev.log")
}

fn root_file(root: &Path) -> PathBuf {
    state_dir(root).join("root")
}

fn dev_up(
    root: &Path,
    _args: &DevLifecycleArgs,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    require_tool("cargo").map_err(|_| CliError::MissingTool("cargo".to_string()))?;
    let dir = state_dir(root);
    fs::create_dir_all(&dir).map_err(|e| CliError::ExternalCommand(e.to_string()))?;

    if pid_file(root).is_file() {
        let pid_text = fs::read_to_string(pid_file(root)).unwrap_or_default();
        let recorded_root = fs::read_to_string(root_file(root)).unwrap_or_default();
        let pid = pid_text.trim().parse::<u32>().ok();
        let same_project = recorded_root.trim() == root.display().to_string();

        if let Some(pid) = pid {
            if process_alive(pid) && same_project {
                return Err(CliError::InvalidInput(format!(
                    "development process is already running with pid {pid}"
                )));
            }
        }

        let _ = fs::remove_file(pid_file(root));
        let _ = fs::remove_file(root_file(root));
    }

    let log =
        fs::File::create(log_file(root)).map_err(|e| CliError::ExternalCommand(e.to_string()))?;
    let log_clone = log
        .try_clone()
        .map_err(|e| CliError::ExternalCommand(e.to_string()))?;

    let cargo = crate::process::tools::find_executable("cargo")
        .ok_or_else(|| CliError::MissingTool("cargo".to_string()))?;
    let mut command = Command::new(cargo);
    command
        .arg("run")
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(log)
        .stderr(log_clone);

    // Do not create a separate process group. Process-group signals can
    // accidentally target the terminal/WSL session on some environments.

    let mut child = command
        .spawn()
        .map_err(|e| CliError::ExternalCommand(e.to_string()))?;
    let pid = child.id();

    fs::write(pid_file(root), pid.to_string())
        .map_err(|e| CliError::ExternalCommand(e.to_string()))?;
    fs::write(root_file(root), root.display().to_string())
        .map_err(|e| CliError::ExternalCommand(e.to_string()))?;

    for _ in 0..20 {
        match child.try_wait() {
            Ok(Some(status)) => {
                let _ = fs::remove_file(pid_file(root));
                let _ = fs::remove_file(root_file(root));
                return Err(CliError::ExternalCommand(format!(
                    "application exited with status {}",
                    status.code().unwrap_or(-1)
                )));
            }
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(error) => {
                let _ = fs::remove_file(pid_file(root));
                let _ = fs::remove_file(root_file(root));
                return Err(CliError::ExternalCommand(error.to_string()));
            }
        }
    }

    // Detach by forgetting the child handle; process continues.
    std::mem::forget(child);

    if !output.quiet {
        output.info(&format!("Development process started with pid {pid}."));
        output.info(&format!("Logs: {}", log_file(root).display()));
    }
    Ok(())
}

fn dev_down(
    root: &Path,
    _args: &DevLifecycleArgs,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    let pid_path = pid_file(root);
    let root_path = root_file(root);
    if !pid_path.is_file() {
        return Err(CliError::InvalidInput(
            "no managed development process is recorded".to_string(),
        ));
    }

    let pid_text =
        fs::read_to_string(&pid_path).map_err(|e| CliError::ExternalCommand(e.to_string()))?;
    let pid = pid_text
        .trim()
        .parse::<u32>()
        .map_err(|_| CliError::InvalidInput("invalid pid state".to_string()))?;

    if root_path.is_file() {
        let recorded_root = fs::read_to_string(&root_path).unwrap_or_default();
        if recorded_root.trim() != root.display().to_string() {
            return Err(CliError::InvalidInput(
                "managed process belongs to a different project".to_string(),
            ));
        }
    }

    terminate_process(pid)?;
    let _ = fs::remove_file(pid_path);
    let _ = fs::remove_file(root_path);

    if !output.quiet {
        output.info(&format!("Stopped development process {pid}."));
    }
    Ok(())
}

fn dev_logs(
    root: &Path,
    _args: &DevLifecycleArgs,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    let path = log_file(root);
    if !path.is_file() {
        return Err(CliError::InvalidInput(
            "no managed development log is available".to_string(),
        ));
    }
    let file = fs::File::open(&path).map_err(|e| CliError::ExternalCommand(e.to_string()))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
    let start = lines.len().saturating_sub(200);
    for line in &lines[start..] {
        output.info(line);
    }
    Ok(())
}

fn terminate_process(pid: u32) -> Result<(), CliError> {
    #[cfg(unix)]
    {
        for child in descendant_pids(pid).into_iter().rev() {
            let _ = Command::new("kill").arg(child.to_string()).status();
        }
        let _ = Command::new("kill").arg(pid.to_string()).status();
    }
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .status();
    }
    Ok(())
}

#[cfg(unix)]
fn descendant_pids(pid: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return result;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(child_pid) = name.to_str().and_then(|value| value.parse::<u32>().ok()) else {
            continue;
        };
        if child_pid == pid {
            continue;
        }
        let Ok(stat) = std::fs::read_to_string(format!("/proc/{}/stat", child_pid)) else {
            continue;
        };
        let Some((_, rest)) = stat.rsplit_once(')') else {
            continue;
        };
        let fields: Vec<&str> = rest.split_whitespace().collect();
        let Some(ppid) = fields.get(1).and_then(|value| value.parse::<u32>().ok()) else {
            continue;
        };
        if ppid == pid {
            result.push(child_pid);
            result.extend(descendant_pids(child_pid));
        }
    }
    result
}

fn process_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/NH"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }
}
