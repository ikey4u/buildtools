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
    set "PROMPT=MSDEV X64 -> "
    set "DevEnvDir=%SCRIPT_DIR%\2022\Common7\IDE\"
    set "EXTERNAL_INCLUDE=%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\include;%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\ATLMFC\include;%SCRIPT_DIR%\2022\VC\Auxiliary\VS\include;%SCRIPT_DIR%\Windows Kits\10\include\10.0.19041.0\ucrt;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\um;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\shared;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\winrt;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\cppwinrt;%SCRIPT_DIR%\Windows Kits\NETFXSDK\4.8\include\um"
    set "ExtensionSdkDir=%SCRIPT_DIR%\Windows Kits\10\Extension SDKs"
    set "INCLUDE=%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\include;%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\ATLMFC\include;%SCRIPT_DIR%\2022\VC\Auxiliary\VS\include;%SCRIPT_DIR%\Windows Kits\10\include\10.0.19041.0\ucrt;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\um;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\shared;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\winrt;%SCRIPT_DIR%\Windows Kits\10\\include\10.0.19041.0\\cppwinrt;%SCRIPT_DIR%\Windows Kits\NETFXSDK\4.8\include\um"
    set "LIB=%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\ATLMFC\lib\x64;%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\lib\x64;%SCRIPT_DIR%\Windows Kits\NETFXSDK\4.8\lib\um\x64;%SCRIPT_DIR%\Windows Kits\10\lib\10.0.19041.0\ucrt\x64;%SCRIPT_DIR%\Windows Kits\10\\lib\10.0.19041.0\\um\x64"
    set "LIBPATH=%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\ATLMFC\lib\x64;%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\lib\x64;%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\lib\x86\store\references;%SCRIPT_DIR%\Windows Kits\10\UnionMetadata\10.0.19041.0;%SCRIPT_DIR%\Windows Kits\10\References\10.0.19041.0"
    set "Path=%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\bin\HostX64\x64;%SCRIPT_DIR%\2022\Common7\IDE\VC\VCPackages;%SCRIPT_DIR%\2022\Common7\IDE\CommonExtensions\Microsoft\TestWindow;%SCRIPT_DIR%\2022\Common7\IDE\CommonExtensions\Microsoft\TeamFoundation\Team Explorer;%SCRIPT_DIR%\2022\MSBuild\Current\bin\Roslyn;C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\;C:\Program Files (x86)\HTML Help Workshop;%SCRIPT_DIR%\2022\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools;%SCRIPT_DIR%\2022\Team Tools\DiagnosticsHub\Collector;%SCRIPT_DIR%\2022\Common7\IDE\Extensions\Microsoft\CodeCoverage.Console;%SCRIPT_DIR%\Windows Kits\10\bin\10.0.19041.0\\x64;%SCRIPT_DIR%\Windows Kits\10\bin\\x64;%SCRIPT_DIR%\2022\\MSBuild\Current\Bin\amd64;%SCRIPT_DIR%\2022\Common7\IDE\;%SCRIPT_DIR%\2022\Common7\Tools\;%SCRIPT_DIR%\2022\VC\Tools\Llvm\x64\bin;%SCRIPT_DIR%\2022\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin;%SCRIPT_DIR%\2022\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%SCRIPT_DIR%\2022\Common7\IDE\VC\Linux\bin\ConnectionManagerExe;%SCRIPT_DIR%\2022\VC\vcpkg;%PATH%"
    set "Platform=x64"
    set "UCRTVersion=10.0.19041.0"
    set "UniversalCRTSdkDir=%SCRIPT_DIR%\Windows Kits\10\"
    set "VCIDEInstallDir=%SCRIPT_DIR%\2022\Common7\IDE\VC\"
    set "VCINSTALLDIR=%SCRIPT_DIR%\2022\VC\"
    set "VCPKG_INSTALLATION_ROOT=C:\vcpkg"
    set "VCPKG_ROOT=%SCRIPT_DIR%\2022\VC\vcpkg"
    set "VCToolsInstallDir=%SCRIPT_DIR%\2022\VC\Tools\MSVC\14.42.34433\"
    set "VCToolsRedistDir=%SCRIPT_DIR%\2022\VC\Redist\MSVC\14.42.34433\"
    set "VCToolsVersion=14.42.34433"
    set "VS170COMNTOOLS=%SCRIPT_DIR%\2022\Common7\Tools\"
    set "VSCMD_ARG_HOST_ARCH=x64"
    set "VSCMD_ARG_TGT_ARCH=x64"
    set "VSCMD_ARG_app_plat=Desktop"
    set "VSCMD_VER=17.12.2"
    set "VSINSTALLDIR=%SCRIPT_DIR%\2022\"
    set "VSSDK150INSTALL=%SCRIPT_DIR%\2022\VSSDK"
    set "VSSDKINSTALL=%SCRIPT_DIR%\2022\VSSDK"
    set "VisualStudioVersion=17.0"
    set "WindowsLibPath=%SCRIPT_DIR%\Windows Kits\10\UnionMetadata\10.0.19041.0;%SCRIPT_DIR%\Windows Kits\10\References\10.0.19041.0"
    set "WindowsSDKLibVersion=10.0.19041.0\"
    set "WindowsSDKVersion=10.0.19041.0\"
    set "WindowsSdkBinPath=%SCRIPT_DIR%\Windows Kits\10\bin\"
    set "WindowsSdkDir=%SCRIPT_DIR%\Windows Kits\10\"
    set "WindowsSdkVerBinPath=%SCRIPT_DIR%\Windows Kits\10\bin\10.0.19041.0\"
)
cmd
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
    match crate::vsenv::get(lnkdir) {
        Ok(mut mp) => {
            for (name, envs) in mp.iter_mut() {
                envs.sort_by_key(|x| x.0.clone());
                let mut f = File::create(install_dir.join(format!("{name}.env")))?;
                for env in envs {
                    _ = f.write_all(format!("{}={}\n", env.0, env.1).as_bytes());
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
