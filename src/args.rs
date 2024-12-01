use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    version, 
    about = "pongactl is a cli program that lets you interact with your ponga-powered tunnels",
    long_about = None,
)]
pub(crate) enum Cli {
    Access(AccessArgs),   
}

#[derive(clap::Args, Debug)]
#[command(
    version, 
    about = "this command connects to (ssh) tunnels from the cli", 
    long_about = None
)]
pub(crate) struct AccessArgs {
    #[arg(long)]
    /// tunnel's entrypoint hostname (i.e. where you started the pongad process)
    pub hostname: String,
    #[arg(short, long)]
    /// tunnel's authserver (i.e. where you started the pongaok process) - defaults to "hostname"
    pub authserver: Option<String>,
}