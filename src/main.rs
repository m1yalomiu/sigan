mod command;

fn main() {
    let command = command::read();
    command::run(command);
}
