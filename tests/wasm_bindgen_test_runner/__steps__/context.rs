use std::path::PathBuf;
use std::process::Command;

pub struct Context {
    assembly: Option<PathBuf>,
    command: Option<Command>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            assembly: None,
            command: None,
        }
    }

    pub fn assembly(&self) -> Option<&PathBuf> {
        self.assembly.as_ref()
    }

    pub fn assembly_set(&mut self, assembly: PathBuf) {
        self.assembly = Some(assembly);
    }

    pub fn command_set(&mut self, command: Command) {
        self.command = Some(command);
    }

    pub fn into_command(self) -> Command {
        self.command.unwrap()
    }
}
