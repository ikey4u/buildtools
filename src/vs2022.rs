use std::{fs::File, io::Write, path::Path};

use anyhow::Context;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::Result;

const MSDEV_SCRIPT: &str = r#"
@echo off

IF DEFINED __MSDEV__ (
    exit /b 0
)

set "SCRIPT_DIR=%~dp0."

setlocal
set "WindowsSdkDir=%SCRIPT_DIR%\Windows Kits\10"
set "WindowsSDKVersion=10.0.19041.0"
set "VCToolsVersion=14.42.34433"
set "VSINSTALLDIR=%SCRIPT_DIR%\2022"
set "VCINSTALLDIR=%SCRIPT_DIR%\2022\VC"
set "__MSDEV__=ON"
if "%1" == "x86" (
    REM x86 Native Tools Command Prompt for VS 2022
    echo TODO
    exit /b 0
) else if "%1" == "cx86" (
    REM x64_x86 Cross Tools Command Prompt for VS 2022
    echo TODO
    exit /b 0
) else if "%1" == "cx64" (
    REM x86_x64 Cross Tools Command Prompt for VS 2022
    echo TODO
    exit /b 0
) else (
    REM x64 Native Tools Command Prompt for VS 2022
    set "PROMPT=MSDEV X64 @ $P$_-> "
    set "VSCMD_ARG_HOST_ARCH=x64"
    set "VSCMD_ARG_TGT_ARCH=x64"
    set "INCLUDE=%VCINSTALLDIR%\Tools\MSVC\%VCToolsVersion%\include;%VCINSTALLDIR%\Auxiliary\VS\include;%WindowsSdkDir%\include\%WindowsSDKVersion%\ucrt;%WindowsSdkDir%\include\%WindowsSDKVersion%\um;%WindowsSdkDir%\include\%WindowsSDKVersion%\shared;%WindowsSdkDir%\include\%WindowsSDKVersion%\winrt;%WindowsSdkDir%\include\%WindowsSDKVersion%\cppwinrt"
    set "LIB=%VCINSTALLDIR%\Tools\MSVC\%VCToolsVersion%\lib\x64;%WindowsSdkDir%\lib\%WindowsSDKVersion%\ucrt\x64;%WindowsSdkDir%\lib\%WindowsSDKVersion%\um\x64"
    set "LIBPATH=%VCINSTALLDIR%\Tools\MSVC\%VCToolsVersion%\lib\x64;%VCINSTALLDIR%\Tools\MSVC\%VCToolsVersion%\lib\x64\store\references;%WindowsSdkDir%\UnionMetadata\%WindowsSDKVersion%;%WindowsSdkDir%\References\%WindowsSDKVersion%"
    set "PATH=%VCINSTALLDIR%\Tools\MSVC\%VCToolsVersion%\bin\HostX64\x64;%WindowsSdkDir%\bin\%WindowsSDKVersion%\x64;%VSINSTALLDIR%\MSBuild\Current\Bin\amd64;%VSINSTALLDIR%\Common7\IDE\;%VSINSTALLDIR%\Common7\Tools\;%VCINSTALLDIR%\Tools\Llvm\x64\bin;%PATH%"
)
cmd /k
endlocal
"#;

pub fn download<P: AsRef<Path>>(location: P, compress: bool) -> Result<()> {
    let location = location.as_ref();

    // download visual studio installer
    let url = "https://aka.ms/vs/17/release/vs_buildtools.exe";
    let resp =
        crate::request(url, None::<&str>).context(format!("download {url}"))?;
    let vs_buildtools = location.join("vs_buildtools.exe");
    let mut f = File::create(&vs_buildtools)?;
    f.write_all(&resp.bytes()?)?;
    // we should drop file explicitly here to fix following Windows error
    //
    //     The process cannot access the file because it is being used by another process. (os error 32)
    //
    drop(f);

    // install visual studio to `ms_buildtools` using installer
    let install_dir = location.join("ms_buildtools");
    let cmd = [
        &format!("{}", vs_buildtools.display()),
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
    zsnip::cmd::CmdBuilder::new(&cmd)?
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
    let install_dir = location.join("ms_buildtools");
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

    // remove non-interested directories
    for dirs in [
        "Windows Kits/10/Windows Performance Toolkit",
        "Windows Kits/10/Testing",
        "Windows Kits/10/References",
        "2022/Common7/IDE/CommonExtensions",
        "2022/Common7/IDE/Extensions",
    ] {
        let mut p = install_dir.clone();
        for dir in dirs.split('/') {
            p = p.join(dir);
        }
        println!("remove directory: {}", p.display());
        _ = std::fs::remove_dir_all(p);
    }


    let mut f = File::create(install_dir.join("msdev.bat"))?;
    f.write_all(MSDEV_SCRIPT.as_bytes())?;

    let lnkdir = r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Visual Studio 2022\Visual Studio Tools\VC";
    let envdir = install_dir.join("vsenv");
    std::fs::create_dir_all(&envdir)?;
    match crate::vsenv::get(lnkdir) {
        Ok(mut mp) => {
            for (name, envs) in mp.iter_mut() {
                // filter out name such as `x86_x64 Cross Tools Command Prompt for VS 2022 (2)`
                if name.ends_with(")") {
                    continue;
                }
                envs.sort_by_key(|x| x.0.clone());
                let mut f = File::create(envdir.join(format!("{name}.txt")))?;
                for env in envs {
                    if env.0.starts_with("GITHUB_") {
                        continue;
                    }
                    if env.0.starts_with("RUNNER_") {
                        continue;
                    }
                    if env.0.starts_with("CARGO") {
                        continue;
                    }
                    if env.0.starts_with("RUST") {
                        continue;
                    }
                    if env.0.starts_with("JAVA") {
                        continue;
                    }
                    if env.0.starts_with("ANDROID") {
                        continue;
                    }
                    if env.0.starts_with("GOROOT") {
                        continue;
                    }
                    if env.0.starts_with("STATS_") {
                        continue;
                    }
                    if env.0.starts_with("__") {
                        continue;
                    }
                    _ = f.write_all(format!("set \"{}={}\"\n", env.0, env.1).as_bytes());
                }
            }
        }
        Err(e) => {
            println!("get visual studio environment variables failed: {e:?}");
        }
    }

    // compress `ms_buildtools` directory
    if compress {
        let ms_buildtool_buf = zsnip::zip::pack(&install_dir)?;
        let mut f = File::create(location.join("ms_buildtools.zip"))?;
        f.write_all(&ms_buildtool_buf)?;
    }

    Ok(())
}
