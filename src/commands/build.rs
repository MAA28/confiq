use std::fs::{create_dir, exists, File};
use std::io::Write;

use log::{info, warn};

use crate::{args::BuildArgs, confiq::Confiq};

pub fn build(args: BuildArgs) {
    let confiq = Confiq::load(&args.path);

    if !exists(&args.build_path.as_path()).expect("Failed to check if build directory exists") {
        create_dir(&args.build_path.as_path()).expect("Failed to create build directory");
    }

    let filename = args.build_path.join("confiqrc.sh");
    let mut file = File::create(&filename).expect("Failed to create file");

    if args.aliases {
        aliases(&mut file, &args, &confiq);
    }
    
    if args.environment_variables {
        environment_variables(&mut file, &args, &confiq);
    }
}

pub fn aliases(confiqrc: &mut File, build_args: &BuildArgs, confiq: &Confiq) {
    let filename = build_args.build_path.join("aliases.sh");
    let file = File::create(&filename).expect("Failed to create file");

    writeln!(confiqrc, "source {}", filename.display()).expect("Failed to write to file");

    info!("Writing aliases to {:?}", filename.display());

    for alias in &confiq.aliases {
        let command = format!("alias {}='{}'", alias.from, alias.to);
        if alias.scope.matching(confiq) {
            info!("Adding command: {:?}", &command);
            writeln!(&file, "{}", command).expect("Failed to write to file");
        } else {
            warn!("Skipping command: {:?}", command);
        }
    }
}

pub fn environment_variables(confiqrc: &mut File, build_args: &BuildArgs, confiq: &Confiq) {
    let filename = build_args.build_path.join("environment_variables.sh");
    let file = File::create(&filename).expect("Failed to create file");

    writeln!(confiqrc, "source {}", filename.display()).expect("Failed to write to file");

    info!("Writing aliases to {:?}", filename.display());

    for environment_variables in &confiq.environment_variables {
        let command = format!("export {}='{}'", environment_variables.key, environment_variables.value);
        if environment_variables.scope.matching(confiq) {
            info!("Adding command: {:?}", &command);
            writeln!(&file, "{}", command).expect("Failed to write to file");
        } else {
            warn!("Skipping command: {:?}", command);
        }
    }
}
