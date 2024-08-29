use std::{fs::File, io::Write, path::Path};

use anyhow::Context;
use clap::Parser;
use rayon::iter::{ParallelBridge, ParallelIterator};
use reqwest::blocking::Response;

pub type Result<T> = anyhow::Result<T, anyhow::Error>;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    compress: bool,
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

    // download visual studio installer
    let url = "https://aka.ms/vs/17/release/vs_buildtools.exe";
    let resp = request(url, None::<&str>).context(format!("download {url}"))?;
    let mut f = File::create("vs_buildtools.exe")?;
    f.write_all(&resp.bytes()?)?;

    // install visual studio to `ms_buildtools` using installer
    let install_dir = cwd.join("ms_buildtools");
    let cmd = [
        "vs_buildtools.exe",
        "--nocache",
        "--quiet",
        "--wait",
        &format!("--installPath {}", install_dir.join("2022").display()),
        "--add Microsoft.VisualStudio.Workload.VCTools",
        "--add Microsoft.Component.MSBuild",
        "--add Microsoft.VisualStudio.Component.Roslyn.Compiler",
        "--add Microsoft.VisualStudio.Component.TextTemplating",
        "--add Microsoft.VisualStudio.Component.VC.CoreIde",
        "--add Microsoft.VisualStudio.Component.VC.Redist.14.Latest",
        "--add Microsoft.VisualStudio.ComponentGroup.NativeDesktop.Core",
        "--add Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
        "--add Microsoft.VisualStudio.ComponentGroup.NativeDesktop.Win81",
        "--add Microsoft.VisualStudio.Component.Windows10SDK.19041",
    ]
    .join(" ");
    snippet::cmd::CmdBuilder::new(&cmd)?
        .stream(true)
        .build()
        .run()
        .context(format!("execute command: {cmd}"))?;

    // copy `Windows Kits` directory into `ms_buildtools`
    let windows_kits_dir = Path::new(
        &std::env::var("ProgramFiles(x86)")
            .context("get `Program Files (x86)` directory")?,
    )
    .join("Windows Kits");
    let options = fs_extra::dir::CopyOptions::new().overwrite(true);
    fs_extra::copy_items(&[&windows_kits_dir], &install_dir, &options)
        .context(format!(
            "copy {} to {}",
            windows_kits_dir.display(),
            install_dir.display()
        ))?;

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
                .context(format!("get parent of {entry:?}"))?
                .join("*.0")
                .display()
        );
        for entry in glob::glob(pat.as_str())
            .context(format!("filter pattern: {pat}"))?
        {
            let entry = entry?;
            if let Some(Some(d)) =
                entry.components().last().map(|x| x.as_os_str().to_str())
            {
                if d != sdk_version {
                    println!("remove directory: {}", entry.display());
                    _ = std::fs::remove_dir_all(&entry);
                }
            }
        }
    }

    // compress `ms_buildtools` directory
    if cli.compress {
        let ms_buildtool_buf = snippet::zip::pack(&install_dir)?;
        let mut f = File::create(cwd.join("ms_buildtools.zip"))?;
        f.write_all(&ms_buildtool_buf)?;
    }

    Ok(())
}
