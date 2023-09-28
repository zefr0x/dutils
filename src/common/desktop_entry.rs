use crate::{Error, Result};
use aho_corasick::AhoCorasick;
use mime::Mime;
use std::{
    collections::HashMap,
    convert::TryFrom,
    ffi::OsString,
    io::IsTerminal,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DesktopEntry {
    pub(crate) name: String,
    pub(crate) exec: String,
    pub(crate) file_name: OsString,
    pub(crate) terminal: bool,
    pub(crate) mimes: Vec<Mime>,
    pub(crate) categories: HashMap<String, ()>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    Launch,
    Open,
}

impl DesktopEntry {
    pub fn exec(&self, mode: Mode, arguments: Vec<String>) -> Result<()> {
        let supports_multiple = self.exec.contains("%F") || self.exec.contains("%U");
        if arguments.is_empty() {
            self.exec_inner(vec![])?
        } else if supports_multiple || mode == Mode::Launch {
            self.exec_inner(arguments)?;
        } else {
            for arg in arguments {
                self.exec_inner(vec![arg])?;
            }
        };

        Ok(())
    }
    fn exec_inner(&self, args: Vec<String>) -> Result<()> {
        let mut cmd = {
            let (cmd, args) = self.get_cmd(args)?;
            let mut cmd = Command::new(cmd);
            cmd.args(args);
            cmd
        };

        if self.terminal && std::io::stdout().is_terminal() {
            cmd.spawn()?.wait()?;
        } else {
            cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;
        }

        Ok(())
    }
    pub fn get_cmd(&self, args: Vec<String>) -> Result<(String, Vec<String>)> {
        let special = AhoCorasick::new(["%f", "%F", "%u", "%U"]).unwrap();

        let mut exec = shlex::split(&self.exec).unwrap();

        // The desktop entry doesn't contain arguments - we make best effort and append them at
        // the end
        if special.is_match(&self.exec) {
            exec = exec
                .into_iter()
                .flat_map(|s| match s.as_str() {
                    "%f" | "%F" | "%u" | "%U" => args.clone(),
                    s if special.is_match(s) => vec![{
                        let mut replaced = String::with_capacity(s.len() + args.len() * 2);
                        special.replace_all_with(s, &mut replaced, |_, _, dst| {
                            dst.push_str(args.clone().join(" ").as_str());
                            false
                        });
                        replaced
                    }],
                    _ => vec![s],
                })
                .collect()
        } else {
            exec.extend_from_slice(&args);
        }

        // If the entry expects a terminal (emulator), but this process is not running in one, we
        // launch a new one.
        if self.terminal && !std::io::stdout().is_terminal() {
            exec = shlex::split(&crate::config::Config::terminal()?)
                .unwrap()
                .into_iter()
                .chain(vec!["-e".to_owned()])
                .chain(exec)
                .collect();
        }

        Ok((exec.remove(0), exec))
    }
}

fn parse_file(path: &Path) -> Option<DesktopEntry> {
    let raw_entry = freedesktop_entry_parser::parse_entry(path).ok()?;
    let section = raw_entry.section("Desktop Entry");

    let mut entry = DesktopEntry {
        file_name: path.file_name()?.to_owned(),
        ..Default::default()
    };

    for attr in section.attrs().filter(|a| a.has_value()) {
        match attr.name {
            "Name" if entry.name.is_empty() => {
                entry.name = attr.value.unwrap().into();
            }
            "Exec" => entry.exec = attr.value.unwrap().into(),
            "MimeType" => {
                entry.mimes = attr
                    .value
                    .unwrap()
                    .split(';')
                    .filter_map(|m| Mime::from_str(m).ok())
                    .collect::<Vec<_>>();
            }
            "Terminal" => entry.terminal = attr.value.unwrap() == "true",
            "Categories" => {
                entry.categories = attr
                    .value
                    .unwrap()
                    .split(';')
                    .filter(|s| !s.is_empty())
                    .map(|cat| (cat.to_owned(), ()))
                    .collect();
            }
            _ => {}
        }
    }

    if !entry.name.is_empty() && !entry.exec.is_empty() {
        Some(entry)
    } else {
        None
    }
}

impl TryFrom<PathBuf> for DesktopEntry {
    type Error = Error;
    fn try_from(path: PathBuf) -> Result<DesktopEntry> {
        parse_file(&path).ok_or(Error::BadEntry(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_exec() {
        let entry = parse_file(Path::new("tests/cmus.desktop")).unwrap();
        assert_eq!(entry.mimes.len(), 2);
        assert_eq!(entry.mimes[0].essence_str(), "audio/mp3");
        assert_eq!(entry.mimes[1].essence_str(), "audio/ogg");
    }
}
