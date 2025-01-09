use integer_storage::save_file;

fn main() -> anyhow::Result<()> {
    save_file("orange.txt", "asxhasx")?;
    Ok(())
}
