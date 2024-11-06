use log::info;

use crate::{args::ScopesArgs, confiq::Confiq};

pub fn scope(args: ScopesArgs) {
    let confiq = Confiq::load(args.path);

    test(&confiq);
}

fn test(confiq: &Confiq) {
    for scope in &confiq.scopes {
        info!("Testing {:?}", scope.0);
        if scope.1.matching() {
            println!("{:?} matches", scope.0);
        } else {
            println!("{:?} does not match", scope.0);
        }
    }
}
