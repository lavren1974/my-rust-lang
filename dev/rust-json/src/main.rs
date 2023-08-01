use reqwest::{Client, Error};

// https://blog.ediri.io/serialize-and-deserialize-data-in-rust-using-serde-and-serdejson

#[tokio::main]
async fn main() -> Result<(), Error> {
    let product = Client::new()
        .get("https://dummyjson.com/products/1")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", product);
    Ok(())
}