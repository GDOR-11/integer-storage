use integer_storage::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    save_file("test2.txt", b"bbananasssss")?;
    println!("{}", read_file("test2.txt").await?);
    Ok(())
}
