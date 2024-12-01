use args::Cli;
use clap::Parser;
use ctl::ctl::Ctl;

mod args;
mod ctl;

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();
    let ctl = Ctl::new(cli);
    ctl
        .connect().await
        .expect("connect");
}