use std::{process::Command, str::from_utf8};

use execute::Execute;
use log::{error, info};
use rust_shell::cmd;

use crate::{args::SourceArgs, confiq::Confiq};

pub fn source(source_args: SourceArgs) {
    let confiq = Confiq::load(source_args.path);
    dbg!(&confiq);

    if source_args.aliases {
        aliases(&confiq);
    }
}

pub fn aliases(confiq: &Confiq) {
    info!("Setting aliases...");

    let output = Command::new("zsh")
        .arg("-c")
        .arg("alias abcdefghijklmop='b'")  // Set the alias in the shell
        .output()
        .expect("Failed to execute command");

    // Check the output for any errors
    if output.status.success() {
        println!("Alias created successfully!");
    } else {
    }
    eprintln!("Failed to create alias: {}", String::from_utf8_lossy(&output.stderr));

    // for alias in &confiq.aliases {
    //     let command = format!("alias {}='{}'", alias.from, alias.to);
    //     if alias.scope.matching(confiq) {
    //         info!("Executing command: {:?}", &command);
    //
    //         cmd!(&command).run();
    //         // if Command::new(&command)
    //         //     .execute_check_exit_status_code(0)
    //         //     .is_err()
    //         // {
    //         //     cmd!(&command).run();
    //         //     error!("Failed to execute command: {:?}", &command);
    //         // }
    //     } else {
    //         info!("Skipping command: {:?}", command);
    //     }
    // }
}
