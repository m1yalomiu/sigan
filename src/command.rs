use std::io;
use std::str::FromStr;

mod task;

#[derive(Debug, PartialEq)]
pub enum Command {
    Start,
    Pause,
    Resume,
    End,
    ListAll
}

pub(crate) fn read() -> Command {
    println!("Please select from the following:");
    println!("1: Start a timer");
    println!("2: Pause a timer");
    println!("3: Resume a timer");
    println!("4: End a timer");
    println!("5: List all timers");

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    return if let Err(()) = Command::from_str(&(command.trim_end())) {
        println!("Unsupported command...");
        read()
    } else {
        Command::from_str(&(command.trim_end())).unwrap()
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "1"  => Ok(Command::Start),
            "2"  => Ok(Command::Pause),
            "3"  => Ok(Command::Resume),
            "4" => Ok(Command::End),
            "5" => Ok(Command::ListAll),
            _      => Err(()),
        }
    }
}

pub(crate) fn run(command: Command) {
    match command {
        Command::Start => { task::start() },
        Command::Pause => { task::pause() },
        Command::Resume => { task::resume() },
        Command::End => { task::end() },
        Command::ListAll => { println!("TBD...") },
    }
}

