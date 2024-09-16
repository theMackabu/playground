use db_proto::DB;
use tokio::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut db = DB::open().await?;

    db.set("beep", "boop").await?;

    println!("{:?}", db.get("beep").await?);

    Ok(())
}
