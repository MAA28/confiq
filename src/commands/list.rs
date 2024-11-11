use crate::{args::{ListArgs, ListCommand}, confiq::Confiq};

pub fn list(args: ListArgs) {
    let confiq = Confiq::load(&args.path);

    match args.command {
        ListCommand::Aliases => aliases(&confiq),
        ListCommand::Scopes => scopes(&confiq),
        ListCommand::EnvironmentVariables => environment_variables(&confiq),
        ListCommand::Scripts => scripts(&confiq),
    }
}

fn scopes(confiq: &Confiq) {
    for scope in &confiq.scopes {
        println!("Scope: {:?}", scope.0);
        println!("\twhoami: {}", scope.1.whoami);
    }
}

fn aliases(confiq: &Confiq) {
    for alias in &confiq.aliases {
        println!("Alias: {:?} -> {:?}", alias.from, alias.to);
        println!("\tScope: {:?}", alias.scope.0);
    } 
}

fn environment_variables(confiq: &Confiq) {
    for env in &confiq.environment_variables {
        println!("Environment Variable: {:?} = {:?}", env.key, env.value);
        println!("\tScope: {:?}", env.scope.0);
    }
}


fn scripts(confiq: &Confiq) {
    for scripts in &confiq.scripts {
        println!("Script: {:?}", scripts.path);
        println!("\tScope: {:?}", scripts.scope.0);
    }
}
