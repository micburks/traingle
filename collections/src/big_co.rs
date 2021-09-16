mod command;
mod company;
mod query;

use company::Company;

pub fn run() {
    let mut company = Company::new();
    // game loop
    loop {
        let command = command::get_command();
        match command {
            command::Command::Exit => break,
            _ => company.run(command),
        }
    }
}
