use std::{error::Error, process::Command};

/// Check if we are in a git repository
///
/// ## Return
/// Result if we are or not in the function.
pub fn is_git_repo() -> bool {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output();
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Delete branch
///
/// # Arguments
/// * `name` - &str
/// * `force` - bool
///
/// # Return
/// `Ok(())` if the function is a success or an Error
pub fn delete_branch(name: &str, force: bool) -> Result<(), Box<dyn Error>> {
    let flag = if force { "-D" } else { "-d" };
    let output = Command::new("git")
        .arg("branch")
        .arg(flag)
        .arg(&name)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to delete branch '{}': {}", name, stderr).into())
    }
}

/// Get all local branch
///
/// # Return
/// `Vec<&str>` if the function is a success or an Error
pub fn get_local_branches() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("git").arg("branch").output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let branches: Vec<String> = stdout
            .lines()
            .map(|line| line.trim_start_matches("* ").trim().to_string())
            .collect();
        Ok(branches)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to get all branch : {}", stderr).into())
    }
}

/// Get all merged branch
///
/// # Argument
/// * `base` - &str
///
/// # Return
/// `Vec<&str>` if the function is a success or an Error
pub fn get_merged_branches(base: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let flag = "--merged";
    let output = Command::new("git")
        .arg("branch")
        .arg(flag)
        .arg(&base)
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let branches: Vec<String> = stdout
            .lines()
            .map(|line| line.trim_start_matches("* ").trim().to_string())
            .collect();
        Ok(branches)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to get all merged branch in '{}': {}", base, stderr).into())
    }
}

/// Change the current branch
///
/// # Argument
/// - `name` - &str
///  
/// # Return
/// Return the success of the commande
pub fn git_checkout(name: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").arg("checkout").arg(&name).output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to checkout branch '{}': {}", name, stderr).into())
    }
}

/// Get the name of the current branch
///
/// # Return
/// The name of the current branch or an Error
pub fn get_current_name() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("Head")
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to get the name of the current branch: {}", stderr).into())
    }
}

/// Get the name of the origin branch
///
/// # Return
/// return the name of the origin branch
pub fn get_origin() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("remote")
        .arg("show")
        .arg("origin")
        .output()?;
    if output.status.success() {
        let raw = String::from_utf8_lossy(&output.stdout);
        let mut stdout = String::new();
        for line in raw.lines() {
            if let Some(stripped) = line.trim().strip_prefix("HEAD branch: ") {
                stdout = stripped.to_string();
            }
        }
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to get the name of the origin branch: {}", stderr).into())
    }
}

/// Get the latest commit of the branch to dertermine if the branch is active
pub fn get_latest_commit(branch: String) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["log", &branch, "--since", "1 month"])
        .output()?;
    if output.status.success() {
        let raw = String::from_utf8_lossy(&output.stdout);
        let mut stdout = String::new();
        for line in raw.lines() {
            if let Some(stripped) = line.trim().strip_prefix("HEAD branch: ") {
                stdout = stripped.to_string();
            }
        }
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "Failed to get the latest commit of the branch {}: {}",
            branch, stderr
        )
        .into())
    }
}
hello world