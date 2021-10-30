mod cli;
use std::env;
use structopt::StructOpt;

fn main() {
    let opt = cli::RustChatCli::from_args();
    println!("opt are {:?}", opt)
}
