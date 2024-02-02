use clap::Parser;

#[derive(Parser)]
#[command(
    author="This is XiaoDeng",
    version=env!("CARGO_PKG_VERSION"),
    about="A CLI Application for Ubuntu PasteBin"
)]
pub struct Arg {

    #[arg(help="The text that will be submitted")]
    pub text: String,

    #[arg(short='s', long="syntax", default_value="text", help="Syntax highlighting type")]
    pub syntax: String,

    #[arg(short='e', long="expiration", default_value="", help="Expiration time")]
    pub expiration: String,

    #[arg(short='p', long="poster", help="Your Ubuntu PasteBin username")]
    pub poster: Option<String>


}

