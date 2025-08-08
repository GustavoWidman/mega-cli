use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use async_read_progress::AsyncReadProgressExt;
use clap::Parser;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::Select;
use itertools::Itertools;
use mega::Node;
use tokio::fs::File;
use tokio_util::compat::TokioAsyncWriteCompatExt;

mod cli;
mod utils;

async fn download(
    mega: &mut mega::Client,
    args: &cli::DownloadArgs,
) -> anyhow::Result<Vec<String>> {
    let file_handle = if args.url.contains("file") && args.url.contains("folder") {
        let regex = regex::Regex::new(r".*\/folder\/(.*)\/file\/(.*)")?;

        match regex.captures(&args.url).map(|c| c.get(2)).flatten() {
            Some(captured) => Some(captured.as_str().to_string()),
            None => None,
        }
    } else {
        None
    };

    let nodes = mega.fetch_public_nodes(&args.url).await?;

    let files: Vec<&Node> = match file_handle {
        Some(file_handle) => vec![
            nodes
                .get_node_by_handle(&file_handle)
                .context("could not find distant node by path")?,
        ],
        None => {
            let options = nodes
                .iter()
                .chain(nodes.roots())
                .filter(|node| node.kind().is_file())
                .map(|node| utils::node::NodeWrapper::new(node))
                .unique()
                .collect::<Vec<_>>();

            if options.len() == 1 {
                options
                    .into_iter()
                    .map(|node| node.into_inner().unwrap())
                    .collect()
            } else {
                let options = options
                    .into_iter()
                    .chain(vec![utils::node::NodeWrapper::new_empty()])
                    .collect::<Vec<_>>();

                Select::new("Select a file to download:", options.clone())
                    .prompt()?
                    .into_inner()
                    .map(|node| vec![node])
                    .unwrap_or_else(|| {
                        options
                            .into_iter()
                            .filter_map(|node| node.into_inner())
                            .collect()
                    })
            }
        }
    };

    let mut created_files = Vec::new();
    for node in files {
        let (reader, writer) = sluice::pipe::pipe();

        let bar = ProgressBar::new(node.size());
        bar.set_style(progress_bar_style());
        bar.set_message(format!("downloading {0}...", node.name()));

        let file = File::create(node.name()).await?;

        let bar = Arc::new(bar);

        let reader = {
            let bar = bar.clone();
            reader.report_progress(Duration::from_millis(100), move |bytes_read| {
                bar.set_position(bytes_read as u64);
            })
        };

        let handle =
            tokio::spawn(async move { futures::io::copy(reader, &mut file.compat_write()).await });
        mega.download_node(node, writer).await?;
        handle.await.unwrap()?;

        bar.finish_with_message(format!("{0} downloaded !", node.name()));

        created_files.push(node.name().to_string());
    }

    Ok(created_files)
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    utils::Logger::init(args.verbosity);

    match args.command {
        cli::Subcommands::Download(args) => {
            let http_client = reqwest::Client::new();

            let mut mega = mega::Client::builder().build(http_client).unwrap();

            match (&args.email, &args.password) {
                (Some(email), Some(password)) => {
                    mega.login(&email, &password, args.mfa.as_ref().map(|s| s.as_str()))
                        .await
                        .unwrap();
                }
                _ => {}
            }

            download(&mut mega, &args).await.unwrap();
        }
    }
}

pub fn progress_bar_style() -> ProgressStyle {
    let template = format!(
        "{}{{bar:30.magenta.bold/magenta/bold}}{} {{percent}} % (ETA {{eta}}, {{decimal_bytes_per_sec}}, {{decimal_bytes}} / {{decimal_total_bytes}}): {{msg}}",
        style("▐").bold().magenta(),
        style("▌").bold().magenta(),
    );

    ProgressStyle::default_bar()
        .progress_chars("▨▨╌")
        .template(template.as_str())
        .unwrap()
}
