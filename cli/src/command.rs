use std::io::{self};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct Command<'a> {
    name: &'a str,
    args: Rc<[&'a str]>,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Command<'a>, Self::Error> {
        let parts = value.split_whitespace();
        let parts = parts.collect::<Rc<_>>();
        let parts = parts.split_first();

        let (name, command_args) = parts.ok_or(())?;

        Ok(Command {
            name,
            args: command_args.iter().map(|arg| arg.to_owned()).collect(),
        })
    }
}

impl Command<'_> {
    pub fn run(&self, paths: Rc<[&Path]>) -> Vec<io::Result<(String, String)>> {
        paths
            .iter()
            .map(|path| {
                std::process::Command::new(self.name)
                    .args(self.args.iter())
                    .arg(path)
                    .output()
                    .map(|std::process::Output { stdout, stderr, .. }| {
                        (
                            String::from_utf8_lossy(&stdout).to_string(),
                            String::from_utf8_lossy(&stderr).to_string(),
                        )
                    })
            })
            .collect()
    }
}
