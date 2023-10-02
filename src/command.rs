use std::io;
use std::str::FromStr;
use crate::command::task::Task;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use colored::Colorize;

mod task;

#[derive(PartialEq)]
pub enum Command {
    Start,
    Pause,
    Resume,
    End,
    ListAll,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "1" => Ok(Command::Start),
            "2" => Ok(Command::Pause),
            "3" => Ok(Command::Resume),
            "4" => Ok(Command::End),
            "5" => Ok(Command::ListAll),
            _ => Err(()),
        }
    }
}

pub(crate) fn run() {
    let tasks: Arc<Mutex<HashMap<String, Task>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        println!("{}", "Please select from the following:".bold().green());
        println!("1: Start a timer");
        println!("2: Pause a timer");
        println!("3: Resume a timer");
        println!("4: End a timer");
        println!("5: List all timers");

        let mut command_input = String::new();
        io::stdin().read_line(&mut command_input).unwrap();
        let command = Command::from_str(&(command_input.trim_end()));

        if let Err(()) = command {
            println!("{}", "Unsupported command...".red());
        } else {
            execute(command.unwrap(), &tasks);
        };
        println!("--------------------------------");
    }
}

fn execute(command: Command, tasks: &Arc<Mutex<HashMap<String, Task>>>) {
    match command {
        Command::Start => { task::start(tasks) }
        Command::Pause => { task::pause(tasks) }
        Command::Resume => { task::resume(tasks) }
        Command::End => { task::end(tasks) }
        Command::ListAll => { task::list_all(tasks) }
    }
}

