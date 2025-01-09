use anyhow::Result;
use std::fs;
use std::process::Command;

pub fn save_file(path: &str, content: &str) -> Result<()> {
    if !Command::new("git")
        .args(["status", "--porcelain"])
        .output()?
        .stdout
        .is_empty()
    {
        return Err(anyhow::anyhow!(
            "Please commit your changes before attempting to save a file"
        ));
    };

    fs::create_dir("numbers")?;
    fs::write(format!("numbers{}{}", std::path::MAIN_SEPARATOR, path), content)?;

    let git = |args: &[&str]| Command::new("git").args(args).status();

    git(&["add", "numbers", "--sparse"])?;
    git(&["commit", "-m", "automatic updates"])?;
    git(&["rebase", "--onto", "origin/main", "HEAD~1", "HEAD"])?;
    git(&["push", "origin", "HEAD:main"])?;
    git(&["checkout", "main"])?;
    git(&["reset", "HEAD~"])?;

    Ok(())
}
