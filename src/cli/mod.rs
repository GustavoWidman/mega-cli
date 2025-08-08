use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser, Debug)]
pub struct Args {
    /// Sets the logger's verbosity level
    #[arg(long, short, default_value_t = LevelFilter::Info)]
    pub verbosity: LevelFilter,

    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Download(DownloadArgs),
}

#[derive(Parser, Debug)]
pub struct DownloadArgs {
    /// The public URL to download from
    #[arg(long, short)]
    pub url: String,

    /// The path to save the downloaded files
    #[arg(long, short)]
    pub path: Option<String>,

    /// The email to use if logging in
    #[arg(long)]
    pub email: Option<String>,
    /// The password to use if logging in
    #[arg(long)]
    pub password: Option<String>,
    /// The two-factor authentication code to use if logging in
    #[arg(long)]
    pub mfa: Option<String>,
}
