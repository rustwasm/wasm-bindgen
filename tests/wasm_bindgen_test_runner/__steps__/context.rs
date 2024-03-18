use std::process::Command;

pub struct Context {
    command: Option<Command>,
}

impl Context {
    pub fn new() -> Self {
        Context { command: None }
    }

    pub fn command_set(&mut self, command: Command) {
        self.command = Some(command);
    }

    pub fn into_command(self) -> Command {
        self.command.unwrap()
    }
}
