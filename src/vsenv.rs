use std::collections::HashMap;

use anyhow::Context;

use crate::Result;

pub fn get<S: AsRef<str>>(
    lnkdir: S,
) -> Result<HashMap<String, Vec<(String, String)>>> {
    let mut mp = HashMap::new();

    for d in std::fs::read_dir(lnkdir.as_ref())? {
        let mut envs = vec![];

        let d = d?.path();
        let stem = d
            .file_stem()
            .context(format!("get name of {}", d.display()))?
            .to_str()
            .context("covnert os string to rust str")?;
        let Ok(shortcut) = lnk::ShellLink::open(&d) else {
            continue;
        };
        let rawcmd = shortcut
            .arguments()
            .as_deref()
            .context(format!("get link command from {}", d.display()))?;
        let cmd = format!("cmd {} && set", rawcmd.replace("/k ", "/c "));
        let (stdout, _) = zsnip::cmd::CmdBuilder::new(&cmd)?
            .build()
            .output(true)
            .context(format!("run command {cmd}"))?;
        envs.push(("VSCMD".to_string(), rawcmd.to_string()));
        for line in stdout.lines() {
            if let Some((key, val)) = line.split_once("=") {
                envs.push((key.to_string(), val.to_string()));
            }
        }

        mp.insert(stem.to_string(), envs);
    }

    Ok(mp)
}
