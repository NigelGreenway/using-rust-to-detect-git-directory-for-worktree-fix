use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;

fn get_git_dir(path: &PathBuf) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .current_dir(path)
        .output()?;

    if output.status.success() {
        let git_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(path.join(git_dir))
    } else {
        Err(anyhow!("Failed to find git directory"))
    }
}

fn get_hooks_path() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    let is_worktree = Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .current_dir(&current_dir)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim() == "true")
            } else {
                None
            }
        })
        .unwrap_or(false);

    let git_dir = if is_worktree {
        let parent = current_dir.parent()
            .ok_or_else(|| anyhow!("No parent directory"))?;
        get_git_dir(&parent.to_path_buf())?
    } else {
        get_git_dir(&current_dir)?
    };

    Ok(git_dir.join("hooks"))
}

fn main() -> Result<()> {
    let hooks_path = get_hooks_path()?;
    println!("Hooks directory: {}", hooks_path.display());
    Ok(())
}
