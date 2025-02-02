use integer_storage::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    save_file("test.txt", b"bananasssss")?;
    println!("kjscdnkjsd");
    println!("{}", read_file("test.txt").await?);
    Ok(())
}
