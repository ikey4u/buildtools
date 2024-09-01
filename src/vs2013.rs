use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

use anyhow::{anyhow, ensure, Context};

use crate::Result;

const CMD_PROFILE_X86: &str = r"
    DevEnvDir=C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\
    ExtensionSdkDir=C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1\ExtensionSDKs
    Framework40Version=v4.0
    FrameworkDir=C:\Windows\Microsoft.NET\Framework\
    FrameworkDIR32=C:\Windows\Microsoft.NET\Framework\
    FrameworkVersion=v4.0.30319
    FrameworkVersion32=v4.0.30319
    INCLUDE=C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\INCLUDE;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\ATLMFC\INCLUDE;C:\Program Files (x86)\Windows Kits\8.1\include\shared;C:\Program Files (x86)\Windows Kits\8.1\include\um;C:\Program Files (x86)\Windows Kits\8.1\include\winrt;
    LIB=C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\LIB;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\ATLMFC\LIB;C:\Program Files (x86)\Windows Kits\8.1\lib\winv6.3\um\x86;
    LIBPATH=C:\Windows\Microsoft.NET\Framework\v4.0.30319;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\LIB;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\ATLMFC\LIB;C:\Program Files (x86)\Windows Kits\8.1\References\CommonConfiguration\Neutral;C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1\ExtensionSDKs\Microsoft.VCLibs\12.0\References\CommonConfiguration\neutral;
    Path=C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files (x86)\Microsoft SDKs\F#\3.1\Framework\v4.0\;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0;C:\Program Files (x86)\MSBuild\12.0\bin;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\BIN;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\Tools;C:\Windows\Microsoft.NET\Framework\v4.0.30319;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\VCPackages;C:\Program Files (x86)\HTML Help Workshop;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Team Tools\Performance Tools;C:\Program Files (x86)\Windows Kits\8.1\bin\x86;C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1A\bin\NETFX 4.5.1 Tools\;
    VCINSTALLDIR=C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\
    VisualStudioVersion=12.0
    VS120COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\Tools\
    VSINSTALLDIR=C:\Program Files (x86)\Microsoft Visual Studio 12.0\
    WindowsSdkDir=C:\Program Files (x86)\Windows Kits\8.1\
    WindowsSDK_ExecutablePath_x64=C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1A\bin\NETFX 4.5.1 Tools\x64\
    WindowsSDK_ExecutablePath_x86=C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1A\bin\NETFX 4.5.1 Tools\
";

const CMD_PROFILE_X64: &str = r"
    CommandPromptType=Native
    ExtensionSdkDir=C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1\ExtensionSDKs
    Framework40Version=v4.0
    FrameworkDir=C:\Windows\Microsoft.NET\Framework64
    FrameworkDIR64=C:\Windows\Microsoft.NET\Framework64
    FrameworkVersion=v4.0.30319
    FrameworkVersion64=v4.0.30319
    INCLUDE=C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\INCLUDE;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\ATLMFC\INCLUDE;C:\Program Files (x86)\Windows Kits\8.1\include\shared;C:\Program Files (x86)\Windows Kits\8.1\include\um;C:\Program Files (x86)\Windows Kits\8.1\include\winrt;
    LIB=C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\LIB\amd64;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\ATLMFC\LIB\amd64;C:\Program Files (x86)\Windows Kits\8.1\lib\winv6.3\um\x64;
    LIBPATH=C:\Windows\Microsoft.NET\Framework64\v4.0.30319;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\LIB\amd64;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\ATLMFC\LIB\amd64;C:\Program Files (x86)\Windows Kits\8.1\References\CommonConfiguration\Neutral;C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1\ExtensionSDKs\Microsoft.VCLibs\12.0\References\CommonConfiguration\neutral;
    Path=C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files (x86)\MSBuild\12.0\bin\amd64;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\BIN\amd64;C:\Windows\Microsoft.NET\Framework64\v4.0.30319;C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\VCPackages;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\Tools;C:\Program Files (x86)\HTML Help Workshop;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Team Tools\Performance Tools\x64;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Team Tools\Performance Tools;C:\Program Files (x86)\Windows Kits\8.1\bin\x64;C:\Program Files (x86)\Windows Kits\8.1\bin\x86;C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1A\bin\NETFX 4.5.1 Tools\x64\;
    Platform=X64
    VCINSTALLDIR=C:\Program Files (x86)\Microsoft Visual Studio 12.0\VC\
    VisualStudioVersion=12.0
    VS110COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 11.0\Common7\Tools\
    VS120COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\Tools\
    VSINSTALLDIR=C:\Program Files (x86)\Microsoft Visual Studio 12.0\
    WindowsSdkDir=C:\Program Files (x86)\Windows Kits\8.1\
    WindowsSDK_ExecutablePath_x64=C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1A\bin\NETFX 4.5.1 Tools\x64\
    WindowsSDK_ExecutablePath_x86=C:\Program Files (x86)\Microsoft SDKs\Windows\v8.1A\bin\NETFX 4.5.1 Tools\
";

/// Create visual studio 2013 package from existed installation
///
/// Download and mount [vs2013.5_ce_enu.iso](http://download.microsoft.com/download/A/A/D/AAD1AA11-FF9A-4B3C-8601-054E89260B78/vs2013.5_ce_enu.iso?type=ISO),
/// then install it using following command:
///
///     start /WAIT "" vs_community.exe /NoRestart /Passive /noweb /NoRefresh /Quiet /Full
///
pub fn create<P: AsRef<Path>>(location: P) -> Result<()> {
    let location = location.as_ref();
    let outdir = location.join("ms_buidtools_vs2013_community");
    if outdir.exists() {
        remove_dir_all(&outdir).context(format!(
            "remove existed visual studio 2013 directory: {}",
            outdir.display()
        ))?;
    } else {
        create_dir_all(&outdir).context(format!(
            "create visual studio 2013 directory: {}",
            outdir.display()
        ))?;
    }

    let dirs = [
        r"C:\Program Files (x86)\MSBuild",
        r"C:\Program Files (x86)\Microsoft SDKs\Windows",
        r"C:\Program Files (x86)\Microsoft Visual Studio 12.0",
        r"C:\Program Files (x86)\Windows Kits",
        r"C:\Windows\Microsoft.NET\Framework",
    ];
    for dir in dirs {
        let dir = Path::new(dir);
        ensure!(
            dir.exists(),
            "required directory {} does not exist",
            dir.display()
        );
        let mut target_dir = outdir.clone();
        for comp in dir
            .components()
            .collect::<Vec<_>>()
            .iter()
            .skip(2)
            .rev()
            .skip(1)
            .rev()
        {
            target_dir = target_dir.join(comp);
        }
        if target_dir == outdir {
            return Err(anyhow!(
                "predefined directory is not valid: {}",
                dir.display()
            ));
        }
        if !target_dir.exists() {
            create_dir_all(&target_dir).context(format!(
                "create directory {}",
                target_dir.display()
            ))?;
        }
        let options = fs_extra::dir::CopyOptions::new().overwrite(true);
        fs_extra::copy_items(&[&dir], &target_dir, &options).context(
            format!("copy {} to {}", dir.display(), target_dir.display()),
        )?;
    }

    let write_profile = |name: &str, profile: &str| -> Result<()> {
        let mut f = File::create(outdir.join(name))?;
        f.write_all("@echo off\r\n".as_bytes())?;
        for line in profile.trim().replace(r"C:\", "%~dp0").lines() {
            let mut line = line.trim().to_string();
            if line.starts_with("Path=") {
                line += ";%PATH%";
            }
            f.write_all(format!("set \"{line}\"\r\n").as_bytes())?;
        }
        Ok(())
    };
    write_profile("x86.bat", CMD_PROFILE_X86)?;
    write_profile("x64.bat", CMD_PROFILE_X64)?;

    Ok(())
}
