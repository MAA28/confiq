use crate::{args::{ListArgs, ListCommand}, confiq::{self, Confiq}};

pub fn list(args: ListArgs) {
    let confiq = Confiq::load(args.path);

    match args.command {
        ListCommand::Aliases => aliases(&confiq),
        ListCommand::Scopes => scopes(&confiq),
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
