use html_parser::Dom;
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.rust-lang.org/";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let dom = Dom::parse(&body).unwrap();
    println!("{:?}", dom.tree_type);

    Ok(())
}
