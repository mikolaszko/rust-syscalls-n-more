use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_to_print = &args[1];
    let mut ls_command = Command::new("ls");
    ls_command.arg(dir_to_print);

    let mut child = ls_command.spawn().unwrap();
    child.wait().unwrap();
}
