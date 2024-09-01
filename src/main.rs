mod vs2013;
mod vs2022;

use std::path::Path;

use anyhow::Context;
use clap::{Parser, Subcommand};
use reqwest::blocking::Response;

pub type Result<T> = anyhow::Result<T, anyhow::Error>;

#[derive(Parser)]
#[clap(author, version, verbatim_doc_comment, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Vs2022 {
        #[arg(long)]
        compress: bool,
    },
    Vs2013,
}

pub fn request<S1: AsRef<str>, S2: AsRef<str>>(
    url: S1,
    proxy: Option<S2>,
) -> Result<Response> {
    let mut builder = reqwest::blocking::Client::builder();
    if let Some(proxy) = proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy.as_ref())?);
    }
    let client = builder.build()?;
    let url = url.as_ref();
    let resp = client.get(url).send().context(format!("GET {url}"))?;
    Ok(resp)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cwd = if let Ok(p) = std::env::var("GITHUB_WORKSPACE") {
        Path::new(&p).to_path_buf()
    } else {
        std::env::current_dir()?
    };

    match &cli.command {
        Some(Commands::Vs2022 { compress }) => {
            crate::vs2022::download(cwd, *compress)?;
        }
        Some(Commands::Vs2013) => {
            crate::vs2013::create(cwd)?;
        }
        None => {}
    }
    Ok(())
}
