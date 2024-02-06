mod argv;

use clap::Parser;
use whoami;
use tokio;
use reqwest;
use edit;

const URL: &str = "https://pastebin.ubuntu.com";

fn get_text() -> String {
    edit::edit("").unwrap()
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let args: argv::Arg = argv::Arg::parse();
    let client: reqwest::Client = reqwest::Client::new();
    let params: [(&str, String); 4] = [
        ("content", args.text.unwrap_or(get_text())),
        ("poster", args.poster.unwrap_or(whoami::username())),
        ("expiration", args.expiration),
        ("syntax", args.syntax)
    ];
    let response: reqwest::Response = client.post(URL)
        .form(&params)
        .send()
        .await?;
    let url = response.url();
    println!("{}", url);
    
    Ok(())
}
