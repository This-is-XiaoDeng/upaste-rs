mod argv;

use clap::Parser;
// use std::io::Result;
use whoami;
use tokio;
use reqwest;


const URL: &str = "https://pastebin.ubuntu.com";

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let args: argv::Arg = argv::Arg::parse();
    let client: reqwest::Client = reqwest::Client::new();
    let params: [(&str, String); 4] = [
        ("content", args.text),
        ("poster", args.poster.unwrap_or(whoami::username())),
        ("expiration", args.expiration),
        ("syntax", args.syntax)
    ];
    let response: reqwest::Response = client.post(URL)
        .form(&params);
        /*.send()
        .await?;
    let url = response.url();
    println!("Url: {}", url);*/
    
    Ok(())
}
