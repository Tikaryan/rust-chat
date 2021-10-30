use structopt::StructOpt;

#[derive(Debug,StructOpt)]
#[structopt(name="rust-chat" ,about="peer to peer chat rust implementation")]
pub struct RustChatCli {
    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: i32,
    #[structopt(short = "u", long = "user")]
    pub peer: String,
}
