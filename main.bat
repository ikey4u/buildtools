@echo on

curl -SL --output vs_buildtools.exe https://aka.ms/vs/17/release/vs_buildtools.exe
start /WAIT "" vs_buildtools.exe --nocache --quiet --wait ^
    --installPath "%CD%\ms_buildtools\2022" ^
    --add Microsoft.VisualStudio.Workload.VCTools ^
    --add Microsoft.Component.MSBuild ^
    --add Microsoft.VisualStudio.Component.Roslyn.Compiler ^
    --add Microsoft.VisualStudio.Component.TextTemplating ^
    --add Microsoft.VisualStudio.Component.VC.CoreIde ^
    --add Microsoft.VisualStudio.Component.VC.Redist.14.Latest ^
    --add Microsoft.VisualStudio.ComponentGroup.NativeDesktop.Core ^
    --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 ^
    --add Microsoft.VisualStudio.ComponentGroup.NativeDesktop.Win81 ^
    --add Microsoft.VisualStudio.Component.Windows10SDK.19041

curl.exe -LO https://github.com/ip7z/7zip/releases/download/22.01/7z2201.exe
7z2201.exe /S /D="%CD%\7z"

xcopy /E "%ProgramFiles(x86)%\Windows Kits" %CD%\ms_buildtools\2022\ >NUL

"%CD%\7z\7z.exe" -tzip ms_buildtools.zip %CD%\ms_buildtools >NUL
