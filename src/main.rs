use std::{fs::File, io::Write, path::Path};

use anyhow::Context;
use rayon::iter::{ParallelBridge, ParallelIterator};
use reqwest::blocking::Response;

pub type Result<T> = anyhow::Result<T, anyhow::Error>;

pub fn request<S1: AsRef<str>, S2: AsRef<str>>(url: S1, proxy: Option<S2>) -> Result<Response> {
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
    let cwd = if let Ok(p) = std::env::var("GITHUB_WORKSPACE") {
        Path::new(&p).to_path_buf()
    } else {
        std::env::current_dir()?
    };
    snippet::cmd::CmdBuilder::new("cmd /c main.bat")?
        .stream(true)
        .cwd(&cwd)
        .build()
        .run()?;

    // remove non-interested SDKs
    let install_dir = cwd.join("ms_buildtools");
    let sdk_version = "10.0.19041.0";
    let (tx, rx) = std::sync::mpsc::channel();
    walkdir::WalkDir::new(&install_dir)
        .into_iter()
        .par_bridge()
        .flatten()
        .filter(|d| {
            let mut accepted = false;
            if d.file_type().is_dir() {
                if let Some(Some(stem)) =
                    d.path().components().last().map(|x| x.as_os_str().to_str())
                {
                    accepted = stem == sdk_version;
                }
            }
            accepted
        })
        .for_each_with(tx, |tx, entry| {
            tx.send(entry).expect("failed to send entry");
        });
    for entry in rx {
        let pat = format!(
            "{}",
            entry
                .path()
                .parent()
                .context(format!("cannot get parent of {entry:?}"))?
                .join("*.0")
                .display()
        );
        println!("filter pattern: {pat}");
        for entry in glob::glob(pat.as_str())? {
            let entry = entry?;
            if let Some(Some(d)) = entry.components().last().map(|x| x.as_os_str().to_str()) {
                if d != sdk_version {
                    println!("remove directory: {}", entry.display());
                    _ = std::fs::remove_dir_all(&entry);
                }
            }
        }
    }

    // compress `ms_buildtools` directory
    let ms_buildtool_buf = snippet::zip::pack(&install_dir)?;
    let mut f = File::create(cwd.join("ms_buildtools.zip"))?;
    f.write_all(&ms_buildtool_buf)?;

    Ok(())
}
