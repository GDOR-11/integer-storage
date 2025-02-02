use std::{fs, path::Path, process::Command};

use anyhow::Result;

/// returns the path to the remote file containing the number
pub fn get_number_filepath(filename: &str) -> String {
    format!("numbers/{filename}.bcd")
}

/// utility function to convert a slice of 10-bit numbers to a vector of 8-bit numbers while
/// preserving the binary representation of the array
fn u10_to_u8(arr: &[u16]) -> Vec<u8> {
    let mut compressed = vec![];
    let mut current_number = 1u8;
    for num in arr {
        for i in 0..10 {
            let bit = ((num >> i) & 1) as u8;
            let overflow;
            (current_number, overflow) = current_number.overflowing_shl(1);
            current_number |= bit;
            if overflow {
                compressed.push(current_number);
                current_number = 1;
            }
        }
    }
    compressed
}

/// reads a file from github
/// * `path` - the full path to the file to read
pub async fn read_file(path: &str) -> Result<String> {
    octocrab::instance()
        .repos("GDOR-11", "integer-storage")
        .get_content()
        .path(path)
        .r#ref("main")
        .send()
        .await?
        .items
        .iter()
        .map(|item| item.decoded_content())
        .collect::<Option<Vec<String>>>()
        .map(|content| content.join(""))
        .ok_or(anyhow::anyhow!(
                "something went wrong while reading the file"
        ))
}

/// saves a file to github
/// * `path` - the full path to the file to save
/// * `content` - the content of the file to save

// I didn't use octocrab in the implementation because it wasn't working
pub fn save_file(path: &str, content: &[u8]) -> Result<()> {
    if !Command::new("git")
        .args(["status", "--porcelain"])
        .output()?
        .stdout
        .is_empty()
    {
        return Err(anyhow::anyhow!(
            "please commit your changes before attempting to save a file"
        ));
    };

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;

    let git = |args: &[&str]| Command::new("git").args(args).status();

    git(&["add", path, "--sparse"])?;
    git(&["commit", "-m", "automatic updates"])?;
    git(&["rebase", "--onto", "origin/main", "HEAD~1", "HEAD"])?;
    git(&["push", "origin", "HEAD:main"])?;
    git(&["checkout", "main"])?;
    git(&["reset", "HEAD~"])?;

    Ok(())
}

/// reads a number from a .bcd file on github, returning it as a string in O(n) time
/// * `filename` - the name of the file to read the number from, without the extension
pub async fn read_number(filename: &str) -> Result<String> {
    let content = read_file(&get_number_filepath(filename)).await?;
    let content = content.as_bytes();

    let mut num = String::new();
    let mut group_value = 0;
    let mut group_index = 0;
    for byte in content {
        for i in (0..8).rev() {
            group_value <<= 1;
            group_value |= (byte >> i) & 1;
            group_index += 1;
            if group_index == 10 {
                num.push_str(&group_value.to_string());
                group_value = 0;
                group_index = 0;
            }
        }
    }

    Ok(num)
}

/// saves a number to github in the .bcd format in O(n) time
/// * `filename` - the name of the file to save the number to, without the extension
/// * `number` - the number to save
pub fn save_number(filename: &str, number: &rug::Integer) -> Result<()> {
    let num = number.to_string_radix(10);
    let mut digit_groups = num.as_bytes().chunks_exact(3);
    let mut bcd = vec![];
    for group in digit_groups.by_ref() {
        let a = group[0] - b'0';
        let b = group[1] - b'0';
        let c = group[2] - b'0';
        bcd.push(100 * a as u16 + 10 * b as u16 + c as u16);
    }
    bcd.push(
        digit_groups
        .remainder()
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &d)| {
            acc + 10u16.pow(i as u32) * (d - b'0') as u16
        }),
    );
    save_file(&get_number_filepath(filename), &u10_to_u8(&bcd))?;
    Ok(())
}
