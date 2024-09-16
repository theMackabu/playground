use db_proto::{clients::Client, Result, DEFAULT_PORT};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = Client::connect(&format!("127.0.0.1:{}", DEFAULT_PORT)).await?;

    client.set("cat", "meow".into()).await?;
    let result = client.get("cat").await?;

    Ok(println!(
        "got value from the server; success={:?}, value = {}",
        result.is_some(),
        String::from_utf8(result.unwrap().to_vec())?
    ))
}
